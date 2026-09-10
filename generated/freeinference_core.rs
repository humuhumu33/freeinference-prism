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
pub struct Preimages {
    pub prompt: alloc::vec::Vec<u8>,
    pub params: alloc::vec::Vec<u8>,
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
    pub residentLabel: alloc::string::String,
    pub servedLabel: alloc::string::String,
    pub sealedLabel: alloc::string::String,
    pub rederiveLabel: alloc::string::String,
    pub identicalLabel: alloc::string::String,
    pub noGpuLabel: alloc::string::String,
    pub offlineLabel: alloc::string::String,
    pub repoLabel: alloc::string::String,
    pub repoUrl: alloc::string::String,
    pub modelLabel: alloc::string::String,
    pub appearanceLabel: alloc::string::String,
    pub darkLabel: alloc::string::String,
    pub lightLabel: alloc::string::String,
    pub immersiveLabel: alloc::string::String,
    pub photoLabel: alloc::string::String,
    pub byLabel: alloc::string::String,
    pub unsplashLabel: alloc::string::String,
    pub wallpapers: alloc::vec::Vec<crate::Wallpaper>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Decision {
    Serve = 0,
    Execute = 1,
    Refuse = 2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wallpaper {
    pub file: alloc::string::String,
    pub label: alloc::string::String,
    pub author: alloc::string::String,
    pub authorUrl: alloc::string::String,
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

pub fn decide(hit: bool, workerAttached: bool) -> crate::Decision {
    match hit {
        false => match workerAttached {
        false => { let _x_55 = crate::Decision::Refuse; _x_55 },
        true => { let _x_56 = crate::Decision::Execute; _x_56 },
    },
        true => { let _x_54 = crate::Decision::Serve; _x_54 },
    }
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

pub fn memoMatches(memo: &crate::Memo, candidates: &[alloc::string::String], promptKappa: &str, paramsKappa: &str) -> bool {
    { let _x_3 = &(memo).promptKappa; { let _x_4 = (_x_3 == promptKappa); match _x_4 {
        false => _x_4,
        true => { let _x_64 = &(memo).paramsKappa; { let _x_65 = (_x_64 == paramsKappa); match _x_65 {
        false => _x_65,
        true => { let _x_69 = &(memo).model; { let _x_70 = intersects(&(_x_69), &(candidates)); _x_70 } },
    } } },
    } } }
}

pub fn optionalDecimal(value: Option<u64>) -> alloc::string::String {
    match value {
        None => alloc::string::String::from("null"),
        Some(val_12) => { let _x_21 = alloc::format!("{}", val_12); _x_21 },
    }
}

pub fn paramsCanonical(request: &crate::Request) -> alloc::string::String {
    { let _x_2 = (request).maxTokens; { let _x_3 = optionalDecimal(_x_2); { let _x_5 = (request).seed; { let _x_6 = optionalDecimal(_x_5); { let _x_8 = temperatureText(&(request)); { let _x_11 = alloc::vec![alloc::string::String::from("}")]; { let _x_12 = { let mut __list = alloc::vec![_x_8]; __list.extend(_x_11); __list }; { let _x_13 = { let mut __list = alloc::vec![alloc::string::String::from(",\"temperature\":")]; __list.extend(_x_12); __list }; { let _x_14 = { let mut __list = alloc::vec![_x_6]; __list.extend(_x_13); __list }; { let _x_15 = { let mut __list = alloc::vec![alloc::string::String::from(",\"seed\":")]; __list.extend(_x_14); __list }; { let _x_16 = { let mut __list = alloc::vec![_x_3]; __list.extend(_x_15); __list }; { let _x_17 = { let mut __list = alloc::vec![alloc::string::String::from("{\"max_tokens\":")]; __list.extend(_x_16); __list }; { let _x_19 = (_x_17).join(&alloc::string::String::from("")); _x_19 } } } } } } } } } } } } }
}

pub fn preimages(request: &crate::Request) -> crate::Preimages {
    { let _x_1 = &(request).messages; { let _x_2 = renderPrompt(&(_x_1)); { let _x_3 = (_x_2).into_bytes(); { let _x_4 = paramsCanonical(&(request)); { let _x_5 = (_x_4).into_bytes(); { let _x_6 = crate::Preimages { prompt: _x_3, params: _x_5 }; _x_6 } } } } } }
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

pub fn temperatureText(request: &crate::Request) -> alloc::string::String {
    { let _x_3 = &(request).temperature; { let _x_5 = _x_3 == ""; match _x_5 {
        false => { let _x_61 = 2147483647; { let _x_62 = { let __value = _x_3; let __delimiter = alloc::string::String::from("\n"); let __maximum = usize::try_from(_x_61).ok(); if __delimiter.is_empty() { None } else { let __fields: alloc::vec::Vec<alloc::string::String> = __value.split(&__delimiter).map(alloc::string::String::from).collect(); __maximum.filter(|__maximum| __fields.len() <= *__maximum).map(|_| __fields) } }; match _x_62 {
        None => alloc::string::String::from(""),
        Some(val_64) => { let _x_66 = (val_64).join(&alloc::string::String::from("\n")); _x_66 },
    } } },
        true => alloc::string::String::from("null"),
    } } }
}

pub fn view() -> crate::View {
    { let _x_27 = crate::Wallpaper { file: alloc::string::String::from("alps.jpg"), label: alloc::string::String::from("Alpine Dawn"), author: alloc::string::String::from("Unsplash"), authorUrl: alloc::string::String::from("https://unsplash.com/?utm_source=Hologram_AI&utm_medium=referral") }; { let _x_32 = crate::Wallpaper { file: alloc::string::String::from("galaxy.jpg"), label: alloc::string::String::from("Galaxy"), author: alloc::string::String::from("Tiago Ferreira"), authorUrl: alloc::string::String::from("https://unsplash.com/@tiago_f_ferreira?utm_source=Hologram_AI&utm_medium=referral") }; { let _x_37 = crate::Wallpaper { file: alloc::string::String::from("aurora.jpg"), label: alloc::string::String::from("Aurora"), author: alloc::string::String::from("Lightscape"), authorUrl: alloc::string::String::from("https://unsplash.com/@lightscape?utm_source=Hologram_AI&utm_medium=referral") }; { let _x_39 = alloc::vec![_x_37]; { let _x_40 = { let mut __list = alloc::vec![_x_32]; __list.extend(_x_39); __list }; { let _x_41 = { let mut __list = alloc::vec![_x_27]; __list.extend(_x_40); __list }; { let _x_42 = crate::View { headline: alloc::string::String::from("Free verified AI inference."), lede: alloc::string::String::from("Ask anything. Every answer is sealed on your device and can be checked again."), promptPlaceholder: alloc::string::String::from("Ask anything"), sendLabel: alloc::string::String::from("Ask"), loadingLabel: alloc::string::String::from("getting the model, once"), residentLabel: alloc::string::String::from("ready"), servedLabel: alloc::string::String::from("Instant, from the seal"), sealedLabel: alloc::string::String::from("Sealed"), rederiveLabel: alloc::string::String::from("Check again"), identicalLabel: alloc::string::String::from("Checked, identical"), noGpuLabel: alloc::string::String::from("This browser cannot run the model. Try Chrome or Edge on a computer."), offlineLabel: alloc::string::String::from("offline, working from your device"), repoLabel: alloc::string::String::from("How it is built"), repoUrl: alloc::string::String::from("https://github.com/humuhumu33/freeinference-prism"), modelLabel: alloc::string::String::from("BitNet 2B, on your device"), appearanceLabel: alloc::string::String::from("Appearance"), darkLabel: alloc::string::String::from("Dark"), lightLabel: alloc::string::String::from("Light"), immersiveLabel: alloc::string::String::from("Immersive"), photoLabel: alloc::string::String::from("Photo"), byLabel: alloc::string::String::from("by"), unsplashLabel: alloc::string::String::from("on Unsplash"), wallpapers: _x_41 }; _x_42 } } } } } } }
}

