#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Provider {
    Local = 0,
    Paid = 1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shard {
    pub label: alloc::string::String,
    pub bytes: u64,
    pub sha256: alloc::string::String,
    pub kappa: alloc::string::String,
    pub objects: alloc::string::String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub model: alloc::string::String,
    pub messages: alloc::vec::Vec<crate::Message>,
    pub maxTokens: Option<u64>,
    pub seed: Option<u64>,
    pub temperature: alloc::string::String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub role: alloc::string::String,
    pub content: alloc::string::String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    pub spec: alloc::string::String,
    pub repo: alloc::string::String,
    pub revision: alloc::string::String,
    pub experts: u64,
    pub tableRows: u64,
    pub shards: alloc::vec::Vec<crate::Shard>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Obj {
    pub kind: alloc::string::String,
    pub label: alloc::string::String,
    pub kappa: alloc::string::String,
    pub bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preimages {
    pub prompt: alloc::vec::Vec<u8>,
    pub params: alloc::vec::Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Route {
    Serve = 0,
    Local = 1,
    Paid = 2,
    NoKey = 3,
    NoGpu = 4,
    PaidOffline = 5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Completion {
    pub id: alloc::string::String,
    pub created: alloc::string::String,
    pub model: alloc::string::String,
    pub text: alloc::string::String,
    pub fingerprint: alloc::string::String,
    pub receipt: alloc::string::String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memo {
    pub model: alloc::vec::Vec<alloc::string::String>,
    pub engineKappa: alloc::string::String,
    pub promptKappa: alloc::string::String,
    pub paramsKappa: alloc::string::String,
    pub outputKappa: alloc::string::String,
    pub receipt: alloc::string::String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct View {
    pub headline: alloc::string::String,
    pub lede: alloc::string::String,
    pub promptPlaceholder: alloc::string::String,
    pub sendLabel: alloc::string::String,
    pub loadingLabel: alloc::string::String,
    pub servedLabel: alloc::string::String,
    pub sealedLabel: alloc::string::String,
    pub rederiveLabel: alloc::string::String,
    pub identicalLabel: alloc::string::String,
    pub noGpuLabel: alloc::string::String,
    pub offlineLabel: alloc::string::String,
    pub modelLabel: alloc::string::String,
    pub appearanceLabel: alloc::string::String,
    pub darkLabel: alloc::string::String,
    pub lightLabel: alloc::string::String,
    pub immersiveLabel: alloc::string::String,
    pub wallpapers: alloc::vec::Vec<crate::Wallpaper>,
    pub localLabel: alloc::string::String,
    pub paidLabel: alloc::string::String,
    pub keyLabel: alloc::string::String,
    pub keyPlaceholder: alloc::string::String,
    pub keySavedLabel: alloc::string::String,
    pub paidOnceLabel: alloc::string::String,
    pub costLabel: alloc::string::String,
    pub freeLabel: alloc::string::String,
    pub noKeyLabel: alloc::string::String,
    pub noCreditLabel: alloc::string::String,
    pub providerBusyLabel: alloc::string::String,
    pub paidOfflineLabel: alloc::string::String,
    pub paidModels: alloc::vec::Vec<crate::PaidModel>,
    pub connectLabel: alloc::string::String,
    pub connectedLabel: alloc::string::String,
    pub listeningLabel: alloc::string::String,
    pub notConnectedLabel: alloc::string::String,
    pub runLabel: alloc::string::String,
    pub verifyLabel: alloc::string::String,
    pub baseUrlLabel: alloc::string::String,
    pub anyKeyLabel: alloc::string::String,
    pub modelIdLabel: alloc::string::String,
    pub testLabel: alloc::string::String,
    pub stayOpenLabel: alloc::string::String,
    pub askLabel: alloc::string::String,
    pub secondTabLabel: alloc::string::String,
    pub copyLabel: alloc::string::String,
    pub copiedLabel: alloc::string::String,
    pub macLabel: alloc::string::String,
    pub windowsLabel: alloc::string::String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaidModel {
    pub id: alloc::string::String,
    pub label: alloc::string::String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Decision {
    Serve = 0,
    Execute = 1,
    Refuse = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: u64,
    pub stop: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wallpaper {
    pub file: alloc::string::String,
    pub label: alloc::string::String,
    pub author: alloc::string::String,
    pub authorUrl: alloc::string::String,
}

pub fn admitPage(listed: &[alloc::string::String], kappa: &str, derived: &str) -> bool {
    { let _x_40 = 2147483647; { let _x_20 = { let __value = kappa; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_40).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; { let _jp_32 = /* jp "_jp_32" inlined at its jump site */ (); match _x_20 {
        None => { let _y_25 = alloc::string::String::from(""); { let _x_26 = anyEqual(&(listed), (_y_25).as_ref()); match _x_26 {
        false => _x_26,
        true => { let _x_51 = (kappa == derived); _x_51 },
    } } },
        Some(val_23) => { let _x_53 = (val_23).join(&alloc::string::String::from("\n")); { let _y_25 = _x_53; { let _x_26 = anyEqual(&(listed), (_y_25).as_ref()); match _x_26 {
        false => _x_26,
        true => { let _x_51 = (kappa == derived); _x_51 },
    } } } },
    } } } }
}

pub fn anyEqual(x_1: &[alloc::string::String], x_2: &str) -> bool {
    match x_1 {
        [] => { let _x_30 = false; _x_30 },
        [head_19, tail_20 @ ..] => { let _x_39 = (head_19 == x_2); match _x_39 {
        false => { let _x_52 = anyEqual(&(tail_20), (x_2).as_ref()); _x_52 },
        true => _x_39,
    } },
    }
}

pub fn contentOf(message: &crate::Message) -> alloc::string::String {
    { let _x_8 = &(message).content; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn createdOf(value: &crate::Completion) -> alloc::string::String {
    { let _x_8 = &(value).created; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn decide(hit: bool, workerAttached: bool) -> crate::Decision {
    match hit {
        false => match workerAttached {
        false => { let _x_55 = crate::Decision::Refuse; _x_55 },
        true => { let _x_56 = crate::Decision::Execute; _x_56 },
    },
        true => { let _x_54 = crate::Decision::Serve; _x_54 },
    }
}

pub fn done() -> alloc::string::String {
    alloc::string::String::from("[DONE]")
}

pub fn encodeCompletion(completion: &crate::Completion) -> alloc::string::String {
    { let _x_3 = idOf(&(completion)); { let _x_4 = escapeJson(_x_3); { let _x_6 = alloc::vec![alloc::string::String::from("\"")]; { let _x_7 = { let mut __list = alloc::vec![_x_4]; __list.extend(_x_6.clone()); __list }; { let _x_8 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_7); __list }; { let _x_10 = (_x_8).join(&alloc::string::String::from("")); { let _x_12 = createdOf(&(completion)); { let _x_14 = modelOf(&(completion)); { let _x_15 = escapeJson(_x_14); { let _x_16 = { let mut __list = alloc::vec![_x_15]; __list.extend(_x_6.clone()); __list }; { let _x_17 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_16); __list }; { let _x_18 = (_x_17).join(&alloc::string::String::from("")); { let _x_20 = fingerprintOf(&(completion)); { let _x_21 = escapeJson(_x_20); { let _x_22 = { let mut __list = alloc::vec![_x_21]; __list.extend(_x_6.clone()); __list }; { let _x_23 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_22); __list }; { let _x_24 = (_x_23).join(&alloc::string::String::from("")); { let _x_26 = textOf(&(completion)); { let _x_27 = escapeJson(_x_26); { let _x_28 = { let mut __list = alloc::vec![_x_27]; __list.extend(_x_6.clone()); __list }; { let _x_29 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_28); __list }; { let _x_30 = (_x_29).join(&alloc::string::String::from("")); { let _x_32 = alloc::vec![alloc::string::String::from(",\"refusal\":null},\"logprobs\":null,\"finish_reason\":\"stop\"}],\"usage\":null}")]; { let _x_33 = { let mut __list = alloc::vec![_x_30]; __list.extend(_x_32); __list }; { let _x_34 = { let mut __list = alloc::vec![alloc::string::String::from(",\"choices\":[{\"index\":0,\"message\":{\"role\":\"assistant\",\"content\":")]; __list.extend(_x_33); __list }; { let _x_35 = { let mut __list = alloc::vec![_x_24]; __list.extend(_x_34); __list }; { let _x_36 = { let mut __list = alloc::vec![alloc::string::String::from(",\"system_fingerprint\":")]; __list.extend(_x_35); __list }; { let _x_37 = { let mut __list = alloc::vec![_x_18]; __list.extend(_x_36); __list }; { let _x_38 = { let mut __list = alloc::vec![alloc::string::String::from(",\"model\":")]; __list.extend(_x_37); __list }; { let _x_39 = { let mut __list = alloc::vec![_x_12]; __list.extend(_x_38); __list }; { let _x_40 = { let mut __list = alloc::vec![alloc::string::String::from(",\"object\":\"chat.completion\",\"created\":")]; __list.extend(_x_39); __list }; { let _x_41 = { let mut __list = alloc::vec![_x_10]; __list.extend(_x_40); __list }; { let _x_42 = { let mut __list = alloc::vec![alloc::string::String::from("{\"id\":")]; __list.extend(_x_41); __list }; { let _x_43 = (_x_42).join(&alloc::string::String::from("")); _x_43 } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } }
}

pub fn encodeDelta(completion: &crate::Completion, delta: alloc::string::String) -> alloc::string::String {
    { let _x_3 = idOf(&(completion)); { let _x_4 = escapeJson(_x_3); { let _x_6 = alloc::vec![alloc::string::String::from("\"")]; { let _x_7 = { let mut __list = alloc::vec![_x_4]; __list.extend(_x_6.clone()); __list }; { let _x_8 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_7); __list }; { let _x_10 = (_x_8).join(&alloc::string::String::from("")); { let _x_12 = createdOf(&(completion)); { let _x_14 = modelOf(&(completion)); { let _x_15 = escapeJson(_x_14); { let _x_16 = { let mut __list = alloc::vec![_x_15]; __list.extend(_x_6.clone()); __list }; { let _x_17 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_16); __list }; { let _x_18 = (_x_17).join(&alloc::string::String::from("")); { let _x_20 = fingerprintOf(&(completion)); { let _x_21 = escapeJson(_x_20); { let _x_22 = { let mut __list = alloc::vec![_x_21]; __list.extend(_x_6.clone()); __list }; { let _x_23 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_22); __list }; { let _x_24 = (_x_23).join(&alloc::string::String::from("")); { let _x_26 = escapeJson(delta); { let _x_27 = { let mut __list = alloc::vec![_x_26]; __list.extend(_x_6.clone()); __list }; { let _x_28 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_27); __list }; { let _x_29 = (_x_28).join(&alloc::string::String::from("")); { let _x_31 = alloc::vec![alloc::string::String::from("},\"finish_reason\":null}]}")]; { let _x_32 = { let mut __list = alloc::vec![_x_29]; __list.extend(_x_31); __list }; { let _x_33 = { let mut __list = alloc::vec![alloc::string::String::from(",\"choices\":[{\"index\":0,\"delta\":{\"content\":")]; __list.extend(_x_32); __list }; { let _x_34 = { let mut __list = alloc::vec![_x_24]; __list.extend(_x_33); __list }; { let _x_35 = { let mut __list = alloc::vec![alloc::string::String::from(",\"system_fingerprint\":")]; __list.extend(_x_34); __list }; { let _x_36 = { let mut __list = alloc::vec![_x_18]; __list.extend(_x_35); __list }; { let _x_37 = { let mut __list = alloc::vec![alloc::string::String::from(",\"model\":")]; __list.extend(_x_36); __list }; { let _x_38 = { let mut __list = alloc::vec![_x_12]; __list.extend(_x_37); __list }; { let _x_39 = { let mut __list = alloc::vec![alloc::string::String::from(",\"object\":\"chat.completion.chunk\",\"created\":")]; __list.extend(_x_38); __list }; { let _x_40 = { let mut __list = alloc::vec![_x_10]; __list.extend(_x_39); __list }; { let _x_41 = { let mut __list = alloc::vec![alloc::string::String::from("{\"id\":")]; __list.extend(_x_40); __list }; { let _x_42 = (_x_41).join(&alloc::string::String::from("")); _x_42 } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } }
}

pub fn encodeError(message: alloc::string::String, kind: alloc::string::String) -> alloc::string::String {
    { let _x_3 = escapeJson(message); { let _x_5 = alloc::vec![alloc::string::String::from("\"")]; { let _x_6 = { let mut __list = alloc::vec![_x_3]; __list.extend(_x_5.clone()); __list }; { let _x_7 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_6); __list }; { let _x_9 = (_x_7).join(&alloc::string::String::from("")); { let _x_11 = escapeJson(kind); { let _x_12 = { let mut __list = alloc::vec![_x_11]; __list.extend(_x_5.clone()); __list }; { let _x_13 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_12); __list }; { let _x_14 = (_x_13).join(&alloc::string::String::from("")); { let _x_16 = alloc::vec![alloc::string::String::from(",\"param\":null,\"code\":null}}")]; { let _x_17 = { let mut __list = alloc::vec![_x_14]; __list.extend(_x_16); __list }; { let _x_18 = { let mut __list = alloc::vec![alloc::string::String::from(",\"type\":")]; __list.extend(_x_17); __list }; { let _x_19 = { let mut __list = alloc::vec![_x_9]; __list.extend(_x_18); __list }; { let _x_20 = { let mut __list = alloc::vec![alloc::string::String::from("{\"error\":{\"message\":")]; __list.extend(_x_19); __list }; { let _x_21 = (_x_20).join(&alloc::string::String::from("")); _x_21 } } } } } } } } } } } } } } }
}

pub fn encodeFinal(completion: &crate::Completion) -> alloc::string::String {
    { let _x_3 = idOf(&(completion)); { let _x_4 = escapeJson(_x_3); { let _x_6 = alloc::vec![alloc::string::String::from("\"")]; { let _x_7 = { let mut __list = alloc::vec![_x_4]; __list.extend(_x_6.clone()); __list }; { let _x_8 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_7); __list }; { let _x_10 = (_x_8).join(&alloc::string::String::from("")); { let _x_12 = createdOf(&(completion)); { let _x_14 = modelOf(&(completion)); { let _x_15 = escapeJson(_x_14); { let _x_16 = { let mut __list = alloc::vec![_x_15]; __list.extend(_x_6.clone()); __list }; { let _x_17 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_16); __list }; { let _x_18 = (_x_17).join(&alloc::string::String::from("")); { let _x_20 = fingerprintOf(&(completion)); { let _x_21 = escapeJson(_x_20); { let _x_22 = { let mut __list = alloc::vec![_x_21]; __list.extend(_x_6.clone()); __list }; { let _x_23 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_22); __list }; { let _x_24 = (_x_23).join(&alloc::string::String::from("")); { let _x_26 = receiptOf(&(completion)); { let _x_27 = escapeJson(_x_26); { let _x_28 = { let mut __list = alloc::vec![_x_27]; __list.extend(_x_6.clone()); __list }; { let _x_29 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_28); __list }; { let _x_30 = (_x_29).join(&alloc::string::String::from("")); { let _x_32 = alloc::vec![alloc::string::String::from("}}")]; { let _x_33 = { let mut __list = alloc::vec![_x_30]; __list.extend(_x_32); __list }; { let _x_34 = { let mut __list = alloc::vec![alloc::string::String::from(",\"choices\":[{\"index\":0,\"delta\":{},\"finish_reason\":\"stop\"}],\"hologram\":{\"receipt\":")]; __list.extend(_x_33); __list }; { let _x_35 = { let mut __list = alloc::vec![_x_24]; __list.extend(_x_34); __list }; { let _x_36 = { let mut __list = alloc::vec![alloc::string::String::from(",\"system_fingerprint\":")]; __list.extend(_x_35); __list }; { let _x_37 = { let mut __list = alloc::vec![_x_18]; __list.extend(_x_36); __list }; { let _x_38 = { let mut __list = alloc::vec![alloc::string::String::from(",\"model\":")]; __list.extend(_x_37); __list }; { let _x_39 = { let mut __list = alloc::vec![_x_12]; __list.extend(_x_38); __list }; { let _x_40 = { let mut __list = alloc::vec![alloc::string::String::from(",\"object\":\"chat.completion.chunk\",\"created\":")]; __list.extend(_x_39); __list }; { let _x_41 = { let mut __list = alloc::vec![_x_10]; __list.extend(_x_40); __list }; { let _x_42 = { let mut __list = alloc::vec![alloc::string::String::from("{\"id\":")]; __list.extend(_x_41); __list }; { let _x_43 = (_x_42).join(&alloc::string::String::from("")); _x_43 } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } }
}

pub fn encodeModels(ids: &[alloc::string::String]) -> alloc::string::String {
    { let _x_2 = modelEntries(&(ids)); { let _x_5 = alloc::vec![alloc::string::String::from("]}")]; { let _x_6 = { let mut __list = alloc::vec![_x_2]; __list.extend(_x_5); __list }; { let _x_7 = { let mut __list = alloc::vec![alloc::string::String::from("{\"object\":\"list\",\"data\":[")]; __list.extend(_x_6); __list }; { let _x_9 = (_x_7).join(&alloc::string::String::from("")); _x_9 } } } } }
}

pub fn encodeOpenRouterRequest(model: alloc::string::String, request: &crate::Request, stream: bool) -> alloc::string::String {
    { let _x_3 = escapeJson(model); { let _x_5 = alloc::vec![alloc::string::String::from("\"")]; { let _x_6 = { let mut __list = alloc::vec![_x_3]; __list.extend(_x_5); __list }; { let _x_7 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_6); __list }; { let _x_9 = (_x_7).join(&alloc::string::String::from("")); { let _x_11 = &(request).messages; { let _x_12 = orMessages(&(_x_11)); { let _x_14 = maxTokensField(&(request)); { let _x_15 = seedField(&(request)); { let _x_16 = temperatureField(&(request)); { let _x_18 = streamText(stream); { let _x_20 = alloc::vec![alloc::string::String::from(",\"usage\":{\"include\":true},\"reasoning\":{\"enabled\":false},\"provider\":{\"require_parameters\":true}}")]; { let _x_21 = { let mut __list = alloc::vec![_x_18]; __list.extend(_x_20); __list }; { let _x_22 = { let mut __list = alloc::vec![alloc::string::String::from(",\"stream\":")]; __list.extend(_x_21); __list }; { let _x_23 = { let mut __list = alloc::vec![_x_16]; __list.extend(_x_22); __list }; { let _x_24 = { let mut __list = alloc::vec![_x_15]; __list.extend(_x_23); __list }; { let _x_25 = { let mut __list = alloc::vec![_x_14]; __list.extend(_x_24); __list }; { let _x_26 = { let mut __list = alloc::vec![alloc::string::String::from("]")]; __list.extend(_x_25); __list }; { let _x_27 = { let mut __list = alloc::vec![_x_12]; __list.extend(_x_26); __list }; { let _x_28 = { let mut __list = alloc::vec![alloc::string::String::from(",\"messages\":[")]; __list.extend(_x_27); __list }; { let _x_29 = { let mut __list = alloc::vec![_x_9]; __list.extend(_x_28); __list }; { let _x_30 = { let mut __list = alloc::vec![alloc::string::String::from("{\"model\":")]; __list.extend(_x_29); __list }; { let _x_31 = (_x_30).join(&alloc::string::String::from("")); _x_31 } } } } } } } } } } } } } } } } } } } } } } }
}

pub fn encodeRole(completion: &crate::Completion) -> alloc::string::String {
    { let _x_3 = idOf(&(completion)); { let _x_4 = escapeJson(_x_3); { let _x_6 = alloc::vec![alloc::string::String::from("\"")]; { let _x_7 = { let mut __list = alloc::vec![_x_4]; __list.extend(_x_6.clone()); __list }; { let _x_8 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_7); __list }; { let _x_10 = (_x_8).join(&alloc::string::String::from("")); { let _x_12 = createdOf(&(completion)); { let _x_14 = modelOf(&(completion)); { let _x_15 = escapeJson(_x_14); { let _x_16 = { let mut __list = alloc::vec![_x_15]; __list.extend(_x_6.clone()); __list }; { let _x_17 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_16); __list }; { let _x_18 = (_x_17).join(&alloc::string::String::from("")); { let _x_20 = fingerprintOf(&(completion)); { let _x_21 = escapeJson(_x_20); { let _x_22 = { let mut __list = alloc::vec![_x_21]; __list.extend(_x_6.clone()); __list }; { let _x_23 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_22); __list }; { let _x_24 = (_x_23).join(&alloc::string::String::from("")); { let _x_26 = alloc::vec![alloc::string::String::from(",\"choices\":[{\"index\":0,\"delta\":{\"role\":\"assistant\",\"content\":\"\"},\"finish_reason\":null}]}")]; { let _x_27 = { let mut __list = alloc::vec![_x_24]; __list.extend(_x_26); __list }; { let _x_28 = { let mut __list = alloc::vec![alloc::string::String::from(",\"system_fingerprint\":")]; __list.extend(_x_27); __list }; { let _x_29 = { let mut __list = alloc::vec![_x_18]; __list.extend(_x_28); __list }; { let _x_30 = { let mut __list = alloc::vec![alloc::string::String::from(",\"model\":")]; __list.extend(_x_29); __list }; { let _x_31 = { let mut __list = alloc::vec![_x_12]; __list.extend(_x_30); __list }; { let _x_32 = { let mut __list = alloc::vec![alloc::string::String::from(",\"object\":\"chat.completion.chunk\",\"created\":")]; __list.extend(_x_31); __list }; { let _x_33 = { let mut __list = alloc::vec![_x_10]; __list.extend(_x_32); __list }; { let _x_34 = { let mut __list = alloc::vec![alloc::string::String::from("{\"id\":")]; __list.extend(_x_33); __list }; { let _x_35 = (_x_34).join(&alloc::string::String::from("")); _x_35 } } } } } } } } } } } } } } } } } } } } } } } } } } }
}

pub fn endpointReady(resident: bool, keyPresent: bool, online: bool) -> bool {
    match resident {
        false => match keyPresent {
        false => keyPresent,
        true => online,
    },
        true => resident,
    }
}

pub fn escapeBackslash(value: alloc::string::String) -> alloc::string::String {
    { let _x_19 = 2147483647; { let _x_12 = { let __value = value; let __delimiter = alloc::string::String::from("\\"); let __maximum = usize::try_from(_x_19).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_12 {
        None => alloc::string::String::from(""),
        Some(val_15) => { let _x_24 = (val_15).join(&alloc::string::String::from("\\\\")); _x_24 },
    } } }
}

pub fn escapeJson(value: alloc::string::String) -> alloc::string::String {
    { let _x_8 = escapeReturn(value); { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\t"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\\t")); _x_25 },
    } } } }
}

pub fn escapeNewline(value: alloc::string::String) -> alloc::string::String {
    { let _x_8 = escapeQuote(value); { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\\n")); _x_25 },
    } } } }
}

pub fn escapeQuote(value: alloc::string::String) -> alloc::string::String {
    { let _x_8 = escapeBackslash(value); { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\""); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\\\"")); _x_25 },
    } } } }
}

pub fn escapeReturn(value: alloc::string::String) -> alloc::string::String {
    { let _x_8 = escapeNewline(value); { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\r"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\\r")); _x_25 },
    } } } }
}

pub fn expertPage(start: u64, length: u64, experts: u64, expert: u64) -> Result<crate::Range, crate::ComputeError> {
    Ok({ let _x_5 = stride(length, experts); { let _x_6 = ((expert) as u64).checked_mul(_x_5).ok_or(crate::ComputeError::MulOverflow)?; { let _x_21 = ((start) as u64).checked_add(_x_6).ok_or(crate::ComputeError::AddOverflow)?; { let _x_8 = 1; { let _x_26 = ((expert) as u64).checked_add(_x_8).ok_or(crate::ComputeError::AddOverflow)?; { let _x_12 = ((_x_26) as u64).checked_mul(_x_5).ok_or(crate::ComputeError::MulOverflow)?; { let _x_30 = ((start) as u64).checked_add(_x_12).ok_or(crate::ComputeError::AddOverflow)?; { let _x_14 = crate::Range { start: _x_21, stop: _x_30 }; _x_14 } } } } } } } })
}

pub fn fingerprintOf(value: &crate::Completion) -> alloc::string::String {
    { let _x_8 = &(value).fingerprint; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn idOf(value: &crate::Completion) -> alloc::string::String {
    { let _x_8 = &(value).id; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn intersects(x_1: &[alloc::string::String], x_2: &[alloc::string::String]) -> bool {
    match x_1 {
        [] => { let _x_28 = false; _x_28 },
        [head_17, tail_18 @ ..] => { let _x_35 = anyEqual(&(x_2), (head_17).as_ref()); match _x_35 {
        false => { let _x_48 = intersects(&(tail_18), &(x_2)); _x_48 },
        true => _x_35,
    } },
    }
}

pub fn manifestRepo(manifest: &crate::Manifest) -> alloc::string::String {
    { let _x_8 = &(manifest).repo; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn manifestRevision(manifest: &crate::Manifest) -> alloc::string::String {
    { let _x_8 = &(manifest).revision; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn manifestSpec(manifest: &crate::Manifest) -> alloc::string::String {
    { let _x_8 = &(manifest).spec; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn maxTokensField(request: &crate::Request) -> alloc::string::String {
    { let _x_16 = (request).maxTokens; match _x_16 {
        None => alloc::string::String::from(""),
        Some(val_19) => { let _x_29 = alloc::format!("{}", val_19); { let _x_31 = alloc::vec![_x_29]; { let _x_32 = { let mut __list = alloc::vec![alloc::string::String::from(",\"max_tokens\":")]; __list.extend(_x_31); __list }; { let _x_34 = (_x_32).join(&alloc::string::String::from("")); _x_34 } } } },
    } }
}

pub fn memoMatches(memo: &crate::Memo, candidates: &[alloc::string::String], promptKappa: &str, paramsKappa: &str) -> bool {
    { let _x_3 = &(memo).promptKappa; { let _x_4 = (_x_3 == promptKappa); match _x_4 {
        false => _x_4,
        true => { let _x_64 = &(memo).paramsKappa; { let _x_65 = (_x_64 == paramsKappa); match _x_65 {
        false => _x_65,
        true => { let _x_69 = &(memo).model; { let _x_70 = intersects(&(_x_69), &(candidates)); _x_70 } },
    } } },
    } } }
}

pub fn modelEntries(x_1: &[alloc::string::String]) -> alloc::string::String {
    match x_1 {
        [] => alloc::string::String::from(""),
        [head_73, tail_74 @ ..] => match tail_74 {
        [] => { let _x_142 = 2147483647; { let _x_143 = { let __value = head_73; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_142).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_143 {
        None => { let _x_146 = modelEntry(alloc::string::String::from("")); _x_146 },
        Some(val_147) => { let _x_151 = (val_147).join(&alloc::string::String::from("\n")); { let _x_149 = modelEntry(_x_151); _x_149 } },
    } } },
        [head_134, tail_135 @ ..] => { let head_134 = head_134.clone(); { let _x_157 = 2147483647; { let _x_158 = { let __value = head_73; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_157).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; { let _jp_159 = /* jp "_jp_159" inlined at its jump site */ (); match _x_158 {
        None => { let _y_160 = alloc::string::String::from(""); { let _x_161 = modelEntry(_y_160); { let _x_162 = modelEntries(&(tail_74)); { let _x_164 = alloc::vec![_x_162]; { let _x_165 = { let mut __list = alloc::vec![_x_161]; __list.extend(_x_164); __list }; { let _x_167 = (_x_165).join(&alloc::string::String::from(",")); _x_167 } } } } } },
        Some(val_170) => { let _x_173 = (val_170).join(&alloc::string::String::from("\n")); { let _y_160 = _x_173; { let _x_161 = modelEntry(_y_160); { let _x_162 = modelEntries(&(tail_74)); { let _x_164 = alloc::vec![_x_162]; { let _x_165 = { let mut __list = alloc::vec![_x_161]; __list.extend(_x_164); __list }; { let _x_167 = (_x_165).join(&alloc::string::String::from(",")); _x_167 } } } } } } },
    } } } } },
    },
    }
}

pub fn modelEntry(id: alloc::string::String) -> alloc::string::String {
    { let _x_3 = escapeJson(id); { let _x_5 = alloc::vec![alloc::string::String::from("\"")]; { let _x_6 = { let mut __list = alloc::vec![_x_3]; __list.extend(_x_5); __list }; { let _x_7 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_6); __list }; { let _x_9 = (_x_7).join(&alloc::string::String::from("")); { let _x_11 = alloc::vec![alloc::string::String::from(",\"object\":\"model\",\"created\":0,\"owned_by\":\"browser\"}")]; { let _x_12 = { let mut __list = alloc::vec![_x_9]; __list.extend(_x_11); __list }; { let _x_13 = { let mut __list = alloc::vec![alloc::string::String::from("{\"id\":")]; __list.extend(_x_12); __list }; { let _x_14 = (_x_13).join(&alloc::string::String::from("")); _x_14 } } } } } } } } }
}

pub fn modelOf(value: &crate::Completion) -> alloc::string::String {
    { let _x_8 = &(value).model; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn objEntry(obj: &crate::Obj) -> alloc::string::String {
    { let _x_2 = objKind(&(obj)); { let _x_3 = escapeJson(_x_2); { let _x_5 = objLabel(&(obj)); { let _x_6 = escapeJson(_x_5); { let _x_7 = objKappa(&(obj)); { let _x_8 = escapeJson(_x_7); { let _x_13 = (obj).bytes; { let _x_14 = alloc::format!("{}", _x_13); { let _x_17 = alloc::vec![alloc::string::String::from("]")]; { let _x_18 = { let mut __list = alloc::vec![_x_14]; __list.extend(_x_17); __list }; { let _x_19 = { let mut __list = alloc::vec![alloc::string::String::from("\",")]; __list.extend(_x_18); __list }; { let _x_20 = { let mut __list = alloc::vec![_x_8]; __list.extend(_x_19); __list }; { let _x_21 = { let mut __list = alloc::vec![alloc::string::String::from("\",\"")]; __list.extend(_x_20); __list }; { let _x_22 = { let mut __list = alloc::vec![_x_6]; __list.extend(_x_21); __list }; { let _x_23 = { let mut __list = alloc::vec![alloc::string::String::from("\",\"")]; __list.extend(_x_22); __list }; { let _x_24 = { let mut __list = alloc::vec![_x_3]; __list.extend(_x_23); __list }; { let _x_25 = { let mut __list = alloc::vec![alloc::string::String::from("[\"")]; __list.extend(_x_24); __list }; { let _x_27 = (_x_25).join(&alloc::string::String::from("")); _x_27 } } } } } } } } } } } } } } } } } }
}

pub fn objKappa(obj: &crate::Obj) -> alloc::string::String {
    { let _x_8 = &(obj).kappa; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn objKind(obj: &crate::Obj) -> alloc::string::String {
    { let _x_8 = &(obj).kind; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn objLabel(obj: &crate::Obj) -> alloc::string::String {
    { let _x_8 = &(obj).label; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn optionalDecimal(value: Option<u64>) -> alloc::string::String {
    match value {
        None => alloc::string::String::from("null"),
        Some(val_12) => { let _x_21 = alloc::format!("{}", val_12); _x_21 },
    }
}

pub fn orMessage(message: &crate::Message) -> alloc::string::String {
    { let _x_3 = roleOf(&(message)); { let _x_4 = escapeJson(_x_3); { let _x_6 = alloc::vec![alloc::string::String::from("\"")]; { let _x_7 = { let mut __list = alloc::vec![_x_4]; __list.extend(_x_6.clone()); __list }; { let _x_8 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_7); __list }; { let _x_10 = (_x_8).join(&alloc::string::String::from("")); { let _x_12 = contentOf(&(message)); { let _x_13 = escapeJson(_x_12); { let _x_14 = { let mut __list = alloc::vec![_x_13]; __list.extend(_x_6.clone()); __list }; { let _x_15 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_14); __list }; { let _x_16 = (_x_15).join(&alloc::string::String::from("")); { let _x_18 = alloc::vec![alloc::string::String::from("}")]; { let _x_19 = { let mut __list = alloc::vec![_x_16]; __list.extend(_x_18); __list }; { let _x_20 = { let mut __list = alloc::vec![alloc::string::String::from(",\"content\":")]; __list.extend(_x_19); __list }; { let _x_21 = { let mut __list = alloc::vec![_x_10]; __list.extend(_x_20); __list }; { let _x_22 = { let mut __list = alloc::vec![alloc::string::String::from("{\"role\":")]; __list.extend(_x_21); __list }; { let _x_23 = (_x_22).join(&alloc::string::String::from("")); _x_23 } } } } } } } } } } } } } } } } }
}

pub fn orMessages(x_1: &[crate::Message]) -> alloc::string::String {
    match x_1 {
        [] => alloc::string::String::from(""),
        [head_29, tail_30 @ ..] => match tail_30 {
        [] => { let _x_49 = orMessage(&(head_29)); _x_49 },
        [head_50, tail_51 @ ..] => { let head_50 = head_50.clone(); { let _x_53 = orMessage(&(head_29)); { let _x_54 = orMessages(&(tail_30)); { let _x_56 = alloc::vec![_x_54]; { let _x_57 = { let mut __list = alloc::vec![_x_53]; __list.extend(_x_56); __list }; { let _x_59 = (_x_57).join(&alloc::string::String::from(",")); _x_59 } } } } } },
    },
    }
}

pub fn pageBytes(rowBytes: u64, rows: u64) -> Result<u64, crate::ComputeError> {
    Ok({ let _x_2 = ((rowBytes) as u64).checked_mul(rows).ok_or(crate::ComputeError::MulOverflow)?; _x_2 })
}

pub fn pageStart(start: u64, rowBytes: u64, rows: u64, index: u64) -> Result<u64, crate::ComputeError> {
    Ok({ let _x_13 = ((rowBytes) as u64).checked_mul(rows).ok_or(crate::ComputeError::MulOverflow)?; { let _x_6 = ((index) as u64).checked_mul(_x_13).ok_or(crate::ComputeError::MulOverflow)?; { let _x_15 = ((start) as u64).checked_add(_x_6).ok_or(crate::ComputeError::AddOverflow)?; _x_15 } } })
}

pub fn paramsCanonical(request: &crate::Request) -> alloc::string::String {
    { let _x_2 = (request).maxTokens; { let _x_3 = optionalDecimal(_x_2); { let _x_5 = (request).seed; { let _x_6 = optionalDecimal(_x_5); { let _x_8 = temperatureText(&(request)); { let _x_11 = alloc::vec![alloc::string::String::from("}")]; { let _x_12 = { let mut __list = alloc::vec![_x_8]; __list.extend(_x_11); __list }; { let _x_13 = { let mut __list = alloc::vec![alloc::string::String::from(",\"temperature\":")]; __list.extend(_x_12); __list }; { let _x_14 = { let mut __list = alloc::vec![_x_6]; __list.extend(_x_13); __list }; { let _x_15 = { let mut __list = alloc::vec![alloc::string::String::from(",\"seed\":")]; __list.extend(_x_14); __list }; { let _x_16 = { let mut __list = alloc::vec![_x_3]; __list.extend(_x_15); __list }; { let _x_17 = { let mut __list = alloc::vec![alloc::string::String::from("{\"max_tokens\":")]; __list.extend(_x_16); __list }; { let _x_19 = (_x_17).join(&alloc::string::String::from("")); _x_19 } } } } } } } } } } } } }
}

pub fn preimages(request: &crate::Request) -> crate::Preimages {
    { let _x_1 = &(request).messages; { let _x_2 = renderPrompt(&(_x_1)); { let _x_3 = (_x_2).into_bytes(); { let _x_4 = paramsCanonical(&(request)); { let _x_5 = (_x_4).into_bytes(); { let _x_6 = crate::Preimages { prompt: _x_3, params: _x_5 }; _x_6 } } } } } }
}

pub fn receiptOf(value: &crate::Completion) -> alloc::string::String {
    { let _x_8 = &(value).receipt; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn renderMessage(message: &crate::Message) -> alloc::string::String {
    { let _x_1 = roleOf(&(message)); { let _x_3 = contentOf(&(message)); { let _x_5 = alloc::vec![_x_3]; { let _x_6 = { let mut __list = alloc::vec![alloc::string::String::from(": ")]; __list.extend(_x_5); __list }; { let _x_7 = { let mut __list = alloc::vec![_x_1]; __list.extend(_x_6); __list }; { let _x_9 = (_x_7).join(&alloc::string::String::from("")); _x_9 } } } } } }
}

pub fn renderPrompt(x_1: &[crate::Message]) -> alloc::string::String {
    match x_1 {
        [] => alloc::string::String::from(""),
        [head_29, tail_30 @ ..] => match tail_30 {
        [] => { let _x_49 = renderMessage(&(head_29)); _x_49 },
        [head_50, tail_51 @ ..] => { let head_50 = head_50.clone(); { let _x_53 = renderMessage(&(head_29)); { let _x_54 = renderPrompt(&(tail_30)); { let _x_56 = alloc::vec![_x_54]; { let _x_57 = { let mut __list = alloc::vec![_x_53]; __list.extend(_x_56); __list }; { let _x_59 = (_x_57).join(&alloc::string::String::from("\n")); _x_59 } } } } } },
    },
    }
}

pub fn roleOf(message: &crate::Message) -> alloc::string::String {
    { let _x_8 = &(message).role; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn rootPreimage(manifest: &crate::Manifest) -> alloc::string::String {
    { let _x_5 = (manifest).experts; { let _x_6 = alloc::format!("{}", _x_5); { let _x_9 = manifestRepo(&(manifest)); { let _x_10 = escapeJson(_x_9); { let _x_12 = alloc::vec![alloc::string::String::from("\"")]; { let _x_13 = { let mut __list = alloc::vec![_x_10]; __list.extend(_x_12.clone()); __list }; { let _x_14 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_13); __list }; { let _x_16 = (_x_14).join(&alloc::string::String::from("")); { let _x_18 = manifestRevision(&(manifest)); { let _x_19 = escapeJson(_x_18); { let _x_20 = { let mut __list = alloc::vec![_x_19]; __list.extend(_x_12.clone()); __list }; { let _x_21 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_20); __list }; { let _x_22 = (_x_21).join(&alloc::string::String::from("")); { let _x_24 = &(manifest).shards; { let _x_25 = shardEntries(&(_x_24)); { let _x_28 = manifestSpec(&(manifest)); { let _x_29 = escapeJson(_x_28); { let _x_30 = { let mut __list = alloc::vec![_x_29]; __list.extend(_x_12.clone()); __list }; { let _x_31 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_30); __list }; { let _x_32 = (_x_31).join(&alloc::string::String::from("")); { let _x_34 = (manifest).tableRows; { let _x_35 = alloc::format!("{}", _x_34); { let _x_37 = alloc::vec![alloc::string::String::from("}")]; { let _x_38 = { let mut __list = alloc::vec![_x_35]; __list.extend(_x_37); __list }; { let _x_39 = { let mut __list = alloc::vec![alloc::string::String::from(",\"table_rows\":")]; __list.extend(_x_38); __list }; { let _x_40 = { let mut __list = alloc::vec![_x_32]; __list.extend(_x_39); __list }; { let _x_41 = { let mut __list = alloc::vec![alloc::string::String::from(",\"spec\":")]; __list.extend(_x_40); __list }; { let _x_42 = { let mut __list = alloc::vec![alloc::string::String::from("]")]; __list.extend(_x_41); __list }; { let _x_43 = { let mut __list = alloc::vec![_x_25]; __list.extend(_x_42); __list }; { let _x_44 = { let mut __list = alloc::vec![alloc::string::String::from(",\"shards\":[")]; __list.extend(_x_43); __list }; { let _x_45 = { let mut __list = alloc::vec![_x_22]; __list.extend(_x_44); __list }; { let _x_46 = { let mut __list = alloc::vec![alloc::string::String::from(",\"revision\":")]; __list.extend(_x_45); __list }; { let _x_47 = { let mut __list = alloc::vec![_x_16]; __list.extend(_x_46); __list }; { let _x_48 = { let mut __list = alloc::vec![alloc::string::String::from(",\"repo\":")]; __list.extend(_x_47); __list }; { let _x_49 = { let mut __list = alloc::vec![_x_6]; __list.extend(_x_48); __list }; { let _x_50 = { let mut __list = alloc::vec![alloc::string::String::from("{\"experts\":")]; __list.extend(_x_49); __list }; { let _x_51 = (_x_50).join(&alloc::string::String::from("")); _x_51 } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } }
}

pub fn route(hit: bool, provider: crate::Provider, gpuReady: bool, keyPresent: bool, online: bool) -> crate::Route {
    match hit {
        false => match provider {
        crate::Provider::Local => match gpuReady {
        false => { let _x_176 = crate::Route::NoGpu; _x_176 },
        true => { let _x_177 = crate::Route::Local; _x_177 },
    },
        crate::Provider::Paid => match keyPresent {
        false => { let _x_181 = crate::Route::NoKey; _x_181 },
        true => match online {
        false => { let _x_182 = crate::Route::PaidOffline; _x_182 },
        true => { let _x_183 = crate::Route::Paid; _x_183 },
    },
    },
    },
        true => { let _x_175 = crate::Route::Serve; _x_175 },
    }
}

pub fn seedField(request: &crate::Request) -> alloc::string::String {
    { let _x_16 = (request).seed; match _x_16 {
        None => alloc::string::String::from(""),
        Some(val_19) => { let _x_29 = alloc::format!("{}", val_19); { let _x_31 = alloc::vec![_x_29]; { let _x_32 = { let mut __list = alloc::vec![alloc::string::String::from(",\"seed\":")]; __list.extend(_x_31); __list }; { let _x_34 = (_x_32).join(&alloc::string::String::from("")); _x_34 } } } },
    } }
}

pub fn sha256Text(shard: &crate::Shard) -> alloc::string::String {
    { let _x_3 = &(shard).sha256; { let _x_5 = _x_3 == ""; match _x_5 {
        false => { let _x_43 = shardSha256(&(shard)); { let _x_44 = escapeJson(_x_43); { let _x_46 = alloc::vec![alloc::string::String::from("\"")]; { let _x_47 = { let mut __list = alloc::vec![_x_44]; __list.extend(_x_46); __list }; { let _x_48 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_47); __list }; { let _x_49 = (_x_48).join(&alloc::string::String::from("")); _x_49 } } } } } },
        true => alloc::string::String::from("null"),
    } } }
}

pub fn shardEntries(shards: &[crate::Shard]) -> alloc::string::String {
    { let _x_2 = shardEntriesFrom(&(shards), alloc::string::String::from("")); _x_2 }
}

pub fn shardEntriesFrom(x_1: &[crate::Shard], x_2: alloc::string::String) -> alloc::string::String {
    match x_1 {
        [] => x_2.clone(),
        [head_26, tail_27 @ ..] => { let _x_55 = x_2.clone() == ""; match _x_55 {
        false => { let _x_73 = shardEntry(&(head_26)); { let _x_75 = alloc::vec![_x_73]; { let _x_76 = { let mut __list = alloc::vec![x_2.clone()]; __list.extend(_x_75); __list }; { let _x_78 = (_x_76).join(&alloc::string::String::from(",")); { let _x_79 = shardEntriesFrom(&(tail_27), _x_78); _x_79 } } } } },
        true => { let _x_80 = shardEntry(&(head_26)); { let _x_81 = shardEntriesFrom(&(tail_27), _x_80); _x_81 } },
    } },
    }
}

pub fn shardEntry(shard: &crate::Shard) -> alloc::string::String {
    { let _x_5 = (shard).bytes; { let _x_6 = alloc::format!("{}", _x_5); { let _x_9 = shardKappa(&(shard)); { let _x_10 = escapeJson(_x_9); { let _x_12 = alloc::vec![alloc::string::String::from("\"")]; { let _x_13 = { let mut __list = alloc::vec![_x_10]; __list.extend(_x_12.clone()); __list }; { let _x_14 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_13); __list }; { let _x_16 = (_x_14).join(&alloc::string::String::from("")); { let _x_18 = shardLabel(&(shard)); { let _x_19 = escapeJson(_x_18); { let _x_20 = { let mut __list = alloc::vec![_x_19]; __list.extend(_x_12.clone()); __list }; { let _x_21 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_20); __list }; { let _x_22 = (_x_21).join(&alloc::string::String::from("")); { let _x_24 = shardObjects(&(shard)); { let _x_25 = escapeJson(_x_24); { let _x_26 = { let mut __list = alloc::vec![_x_25]; __list.extend(_x_12.clone()); __list }; { let _x_27 = { let mut __list = alloc::vec![alloc::string::String::from("\"")]; __list.extend(_x_26); __list }; { let _x_28 = (_x_27).join(&alloc::string::String::from("")); { let _x_30 = sha256Text(&(shard)); { let _x_32 = alloc::vec![alloc::string::String::from("}")]; { let _x_33 = { let mut __list = alloc::vec![_x_30]; __list.extend(_x_32); __list }; { let _x_34 = { let mut __list = alloc::vec![alloc::string::String::from(",\"sha256\":")]; __list.extend(_x_33); __list }; { let _x_35 = { let mut __list = alloc::vec![_x_28]; __list.extend(_x_34); __list }; { let _x_36 = { let mut __list = alloc::vec![alloc::string::String::from(",\"objects\":")]; __list.extend(_x_35); __list }; { let _x_37 = { let mut __list = alloc::vec![_x_22]; __list.extend(_x_36); __list }; { let _x_38 = { let mut __list = alloc::vec![alloc::string::String::from(",\"name\":")]; __list.extend(_x_37); __list }; { let _x_39 = { let mut __list = alloc::vec![_x_16]; __list.extend(_x_38); __list }; { let _x_40 = { let mut __list = alloc::vec![alloc::string::String::from(",\"kappa\":")]; __list.extend(_x_39); __list }; { let _x_41 = { let mut __list = alloc::vec![_x_6]; __list.extend(_x_40); __list }; { let _x_42 = { let mut __list = alloc::vec![alloc::string::String::from("{\"bytes\":")]; __list.extend(_x_41); __list }; { let _x_43 = (_x_42).join(&alloc::string::String::from("")); _x_43 } } } } } } } } } } } } } } } } } } } } } } } } } } } } } } }
}

pub fn shardKappa(shard: &crate::Shard) -> alloc::string::String {
    { let _x_8 = &(shard).kappa; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn shardLabel(shard: &crate::Shard) -> alloc::string::String {
    { let _x_8 = &(shard).label; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn shardObjects(shard: &crate::Shard) -> alloc::string::String {
    { let _x_8 = &(shard).objects; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn shardSha256(shard: &crate::Shard) -> alloc::string::String {
    { let _x_8 = &(shard).sha256; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn streamText(stream: bool) -> alloc::string::String {
    match stream {
        false => alloc::string::String::from("false"),
        true => alloc::string::String::from("true"),
    }
}

pub fn stride(length: u64, experts: u64) -> u64 {
    { let _x_2 = 0; { let _x_5 = if experts == 0 { _x_2 } else { length / experts }; _x_5 } }
}

pub fn tablePage(start: u64, stop: u64, rowBytes: u64, rows: u64, index: u64) -> Result<crate::Range, crate::ComputeError> {
    Ok({ let _x_1 = pageStart(start, rowBytes, rows, index)?; { let _x_20 = ((rowBytes) as u64).checked_mul(rows).ok_or(crate::ComputeError::MulOverflow)?; { let _x_22 = ((_x_1) as u64).checked_add(_x_20).ok_or(crate::ComputeError::AddOverflow)?; { let _x_7 = (_x_22 <= stop); match _x_7 {
        false => { let _x_41 = crate::Range { start: _x_1, stop: stop }; _x_41 },
        true => { let _x_42 = crate::Range { start: _x_1, stop: _x_22 }; _x_42 },
    } } } } })
}

pub fn temperatureField(request: &crate::Request) -> alloc::string::String {
    { let _x_3 = &(request).temperature; { let _x_5 = _x_3 == ""; match _x_5 {
        false => { let _x_83 = 2147483647; { let _x_84 = { let __value = _x_3; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_83).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; { let _jp_85 = /* jp "_jp_85" inlined at its jump site */ (); match _x_84 {
        None => { let _y_86 = alloc::string::String::from(""); { let _x_88 = alloc::vec![_y_86]; { let _x_89 = { let mut __list = alloc::vec![alloc::string::String::from(",\"temperature\":")]; __list.extend(_x_88); __list }; { let _x_90 = (_x_89).join(&alloc::string::String::from("")); _x_90 } } } },
        Some(val_92) => { let _x_94 = (val_92).join(&alloc::string::String::from("\n")); { let _y_86 = _x_94; { let _x_88 = alloc::vec![_y_86]; { let _x_89 = { let mut __list = alloc::vec![alloc::string::String::from(",\"temperature\":")]; __list.extend(_x_88); __list }; { let _x_90 = (_x_89).join(&alloc::string::String::from("")); _x_90 } } } } },
    } } } },
        true => alloc::string::String::from(""),
    } } }
}

pub fn temperatureText(request: &crate::Request) -> alloc::string::String {
    { let _x_3 = &(request).temperature; { let _x_5 = _x_3 == ""; match _x_5 {
        false => { let _x_61 = 2147483647; { let _x_62 = { let __value = _x_3; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_61).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_62 {
        None => alloc::string::String::from(""),
        Some(val_64) => { let _x_66 = (val_64).join(&alloc::string::String::from("\n")); _x_66 },
    } } },
        true => alloc::string::String::from("null"),
    } } }
}

pub fn textOf(value: &crate::Completion) -> alloc::string::String {
    { let _x_8 = &(value).text; { let _x_20 = 2147483647; { let _x_13 = { let __value = _x_8; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_20).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_13 {
        None => alloc::string::String::from(""),
        Some(val_16) => { let _x_25 = (val_16).join(&alloc::string::String::from("\n")); _x_25 },
    } } } }
}

pub fn view() -> crate::View {
    { let _x_21 = crate::Wallpaper { file: alloc::string::String::from("alps.jpg"), label: alloc::string::String::from("Alpine Dawn"), author: alloc::string::String::from("Unsplash"), authorUrl: alloc::string::String::from("https://unsplash.com/?utm_source=Hologram_AI&utm_medium=referral") }; { let _x_26 = crate::Wallpaper { file: alloc::string::String::from("galaxy.jpg"), label: alloc::string::String::from("Galaxy"), author: alloc::string::String::from("Tiago Ferreira"), authorUrl: alloc::string::String::from("https://unsplash.com/@tiago_f_ferreira?utm_source=Hologram_AI&utm_medium=referral") }; { let _x_31 = crate::Wallpaper { file: alloc::string::String::from("aurora.jpg"), label: alloc::string::String::from("Aurora"), author: alloc::string::String::from("Lightscape"), authorUrl: alloc::string::String::from("https://unsplash.com/@lightscape?utm_source=Hologram_AI&utm_medium=referral") }; { let _x_33 = alloc::vec![_x_31]; { let _x_34 = { let mut __list = alloc::vec![_x_26]; __list.extend(_x_33); __list }; { let _x_35 = { let mut __list = alloc::vec![_x_21]; __list.extend(_x_34); __list }; { let _x_49 = crate::PaidModel { id: alloc::string::String::from("qwen/qwen3.8-flash"), label: alloc::string::String::from("Qwen 3.8 Flash") }; { let _x_52 = crate::PaidModel { id: alloc::string::String::from("deepseek/deepseek-v4.1-flash"), label: alloc::string::String::from("DeepSeek V4.1 Flash") }; { let _x_55 = crate::PaidModel { id: alloc::string::String::from("nvidia/nemotron-3.5-lightning:free"), label: alloc::string::String::from("Nemotron 3.5, free") }; { let _x_57 = alloc::vec![_x_55]; { let _x_58 = { let mut __list = alloc::vec![_x_52]; __list.extend(_x_57); __list }; { let _x_59 = { let mut __list = alloc::vec![_x_49]; __list.extend(_x_58); __list }; { let _x_77 = crate::View { headline: alloc::string::String::from("Own Your Ideas"), lede: alloc::string::String::from("Seamlessly build, run, share and earn from your serverless AI applications."), promptPlaceholder: alloc::string::String::from("Ask anything"), sendLabel: alloc::string::String::from("Ask"), loadingLabel: alloc::string::String::from("getting the model, once"), servedLabel: alloc::string::String::from("Instant, from the seal"), sealedLabel: alloc::string::String::from("Sealed"), rederiveLabel: alloc::string::String::from("Check again"), identicalLabel: alloc::string::String::from("Checked, identical"), noGpuLabel: alloc::string::String::from("This browser cannot run the model. Try Chrome or Edge on a computer."), offlineLabel: alloc::string::String::from("offline, working from your device"), modelLabel: alloc::string::String::from("BitNet 2B, on your device"), appearanceLabel: alloc::string::String::from("Appearance"), darkLabel: alloc::string::String::from("Dark"), lightLabel: alloc::string::String::from("Light"), immersiveLabel: alloc::string::String::from("Immersive"), wallpapers: _x_35, localLabel: alloc::string::String::from("On your device"), paidLabel: alloc::string::String::from("Paid"), keyLabel: alloc::string::String::from("OpenRouter key"), keyPlaceholder: alloc::string::String::from("Paste your OpenRouter key"), keySavedLabel: alloc::string::String::from("Key kept on this device"), paidOnceLabel: alloc::string::String::from("Paid once, then free from the seal"), costLabel: alloc::string::String::from("Paid"), freeLabel: alloc::string::String::from("Free"), noKeyLabel: alloc::string::String::from("Add your OpenRouter key to use paid models"), noCreditLabel: alloc::string::String::from("Your OpenRouter account has no credit"), providerBusyLabel: alloc::string::String::from("That model is busy right now. Try again or pick another"), paidOfflineLabel: alloc::string::String::from("Paid models need the network"), paidModels: _x_59, connectLabel: alloc::string::String::from("Connect"), connectedLabel: alloc::string::String::from("Connected"), listeningLabel: alloc::string::String::from("Listening for the relay"), notConnectedLabel: alloc::string::String::from("Not connected"), runLabel: alloc::string::String::from("Run this once, on this computer"), verifyLabel: alloc::string::String::from("Verify the file"), baseUrlLabel: alloc::string::String::from("Base URL"), anyKeyLabel: alloc::string::String::from("Any key works, for example local"), modelIdLabel: alloc::string::String::from("Model"), testLabel: alloc::string::String::from("Send a test request"), stayOpenLabel: alloc::string::String::from("Nothing leaves your device. Keep this tab open."), askLabel: alloc::string::String::from("Your browser may ask to let this page reach your computer."), secondTabLabel: alloc::string::String::from("Another tab is already serving"), copyLabel: alloc::string::String::from("Copy"), copiedLabel: alloc::string::String::from("Copied"), macLabel: alloc::string::String::from("macOS / Linux"), windowsLabel: alloc::string::String::from("Windows") }; _x_77 } } } } } } } } } } } } }
}

