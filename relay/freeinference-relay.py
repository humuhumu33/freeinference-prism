#!/usr/bin/env python3
"""freeinference relay: one file, no dependencies, the smallest bridge from a native
harness to the model running in your browser tab.

A browser tab cannot listen on a port, and Hermes, OpenClaw and every OpenAI client
expect http://host:port/v1. This script listens on 127.0.0.1:11435 and forwards each
request to the freeinference page open in your browser, which executes it on your
GPU, seals it, and streams the answer back. Nothing is computed here, nothing is
stored here, and nothing leaves your machine.

    python freeinference-relay.py            # then open https://humuhumu33.github.io/freeinference-prism/
    OPENAI_BASE_URL=http://127.0.0.1:11435/v1  OPENAI_API_KEY=local

Wire shapes are the page's, which are the Lean verified model's; this file only moves bytes.
"""
import json, os, queue, sys, threading, time, uuid
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

HOST, PORT = "127.0.0.1", int(os.environ.get("FREEINFERENCE_RELAY_PORT", "11435"))
PAGE_ORIGINS = {"https://humuhumu33.github.io", "http://localhost:8090", "http://127.0.0.1:8090"}
POLL_SECONDS, TIMEOUT_SECONDS = 25, int(os.environ.get("FREEINFERENCE_RELAY_TIMEOUT", "180"))

jobs = queue.Queue()            # jobs waiting for a tab
running = {}                    # id -> {"frames": queue.Queue, "request": dict}
lock = threading.Lock()
last_poll = 0.0
models_cache = {"bytes": None, "at": 0.0}


def tab_attached():
    return time.time() - last_poll < 2 * POLL_SECONDS


class Handler(BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, fmt, *args):
        sys.stderr.write("relay  %s\n" % (fmt % args))

    # ---- helpers
    def cors(self):
        origin = self.headers.get("Origin", "")
        if origin in PAGE_ORIGINS:
            self.send_header("Access-Control-Allow-Origin", origin)
            self.send_header("Access-Control-Allow-Headers", "content-type")
            self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")

    def body(self):
        length = int(self.headers.get("Content-Length") or 0)
        return self.rfile.read(length) if length else b""

    def reply(self, status, payload, content_type="application/json", extra=()):
        data = payload if isinstance(payload, bytes) else json.dumps(payload).encode()
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(data)))
        for k, v in extra:
            self.send_header(k, v)
        self.cors()
        self.end_headers()
        self.wfile.write(data)

    def error(self, status, message):
        self.reply(status, {"error": {"message": message, "type": "server_error", "param": None, "code": None}})

    def do_OPTIONS(self):
        self.send_response(204); self.cors(); self.send_header("Content-Length", "0"); self.end_headers()

    # ---- the harness side
    def do_GET(self):
        global last_poll
        if self.path == "/v1/models":
            # The tab answers this too: ask it, so the list is the page's, from the model.
            if not tab_attached():
                return self.reply(200, {"object": "list", "data": []})
            return self.forward({"op": "models"}, stream=False)
        if self.path == "/tab/next":
            last_poll = time.time()
            deadline = time.time() + POLL_SECONDS
            while time.time() < deadline:
                try:
                    job = jobs.get(timeout=0.5)
                except queue.Empty:
                    continue
                last_poll = time.time()
                return self.reply(200, job)
            self.send_response(204); self.cors(); self.send_header("Content-Length", "0"); self.end_headers()
            return
        if self.path == "/tab/hello":
            return self.reply(200, {"relay": "freeinference", "port": PORT})
        if self.path in ("/", "/healthz"):
            return self.reply(200, {"relay": "freeinference", "tab": tab_attached(), "base_url": f"http://{HOST}:{PORT}/v1"})
        self.error(404, "not found")

    def do_POST(self):
        if self.path == "/v1/chat/completions":
            try:
                request = json.loads(self.body() or b"{}")
            except ValueError:
                return self.error(400, "request is not JSON")
            if not tab_attached():
                return self.error(503, "no browser tab is serving: open https://humuhumu33.github.io/freeinference-prism/ and leave it open")
            return self.forward({"op": "chat", "request": request}, stream=bool(request.get("stream")))
        if self.path.startswith("/tab/"):
            parts = self.path.split("/")
            if len(parts) != 4:
                return self.error(404, "not found")
            job_id, action = parts[2], parts[3]
            with lock:
                job = running.get(job_id)
            if not job:
                return self.error(404, "no running job")
            try:
                payload = json.loads(self.body() or b"{}")
            except ValueError:
                return self.error(400, "not JSON")
            if action == "head":           # the stream's headers, before its first frame
                job["frames"].put(("head", payload.get("headers", {})))
                return self.reply(202, {})
            if action == "frame":          # one wire frame, bytes already encoded by the page
                job["frames"].put(("frame", payload.get("bytes", "")))
                return self.reply(202, {})
            if action == "result":         # the whole response, bytes already encoded
                job["frames"].put(("result", payload.get("bytes", ""), payload.get("headers", {})))
                return self.reply(202, {})
            if action == "fail":
                job["frames"].put(("fail", payload.get("message", "the tab failed")))
                return self.reply(202, {})
        self.error(404, "not found")

    # ---- forward one job to the tab and relay what comes back
    def forward(self, job, stream):
        job_id = str(uuid.uuid4())
        frames = queue.Queue()
        with lock:
            running[job_id] = {"frames": frames, "request": job}
        jobs.put({"id": job_id, **job})
        try:
            if stream:
                deadline = time.time() + TIMEOUT_SECONDS
                headed = False
                while time.time() < deadline:
                    try:
                        item = frames.get(timeout=1)
                    except queue.Empty:
                        continue
                    if not headed:
                        headed = True
                        self.send_response(200)
                        self.send_header("Content-Type", "text/event-stream")
                        self.send_header("Cache-Control", "no-cache")
                        for k, v in (item[1] if item[0] == "head" else {}).items():
                            self.send_header(k, str(v))
                        self.cors()
                        self.end_headers()
                    if item[0] == "head":
                        continue
                    if item[0] == "frame":
                        self.wfile.write(("data: " + item[1] + "\n\n").encode()); self.wfile.flush()
                        if item[1] == "[DONE]":
                            return
                    elif item[0] == "result":
                        self.wfile.write(("data: " + item[1] + "\n\n").encode()); self.wfile.flush()
                        return
                    else:
                        self.wfile.write(("data: " + json.dumps({"error": {"message": item[1], "type": "server_error"}}) + "\n\n").encode())
                        self.wfile.flush(); return
                if not headed:
                    return self.error(504, f"no answer from the tab within {TIMEOUT_SECONDS} s")
                self.wfile.write(b"data: [DONE]\n\n"); self.wfile.flush()
            else:
                deadline = time.time() + TIMEOUT_SECONDS
                while time.time() < deadline:
                    try:
                        item = frames.get(timeout=1)
                    except queue.Empty:
                        continue
                    if item[0] == "result":
                        return self.reply(200, item[1].encode(), extra=tuple((k, str(v)) for k, v in item[2].items()))
                    if item[0] == "fail":
                        return self.error(502, item[1])
                self.error(504, f"no answer from the tab within {TIMEOUT_SECONDS} s")
        finally:
            with lock:
                running.pop(job_id, None)


if __name__ == "__main__":
    server = ThreadingHTTPServer((HOST, PORT), Handler)
    print(f"freeinference relay on http://{HOST}:{PORT}/v1")
    print("open https://humuhumu33.github.io/freeinference-prism/ in your browser and leave it open")
    print(f"then point any OpenAI client at OPENAI_BASE_URL=http://{HOST}:{PORT}/v1 with any key")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
