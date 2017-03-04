pub struct Webhook {
    pub id: String,
    pub token: String,
}

impl Webhook {
    pub fn new(id: String, token: String) -> Webhook {
        Webhook {
            id: id,
            token: token,
        }
    }
}

#[derive(Serialize,Debug)]
pub struct ExecuteData {
    // At least one of these three required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embeds>>,

    // Optional
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}

impl ExecuteData {
    pub fn new(content: ExecuteContent) -> ExecuteData {
        let mut execute_data = ExecuteData {
            content: None,
            file: None,
            embeds: None,
            username: None,
            avatar_url: None,
            tts: None,
        };

        match content {
            ExecuteContent::Content(c) => execute_data.content = Some(c),
            ExecuteContent::File(f) => execute_data.file = Some(f),
            ExecuteContent::Embeds(e) => execute_data.embeds = Some(e),
        }

        execute_data
    }

    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    pub fn avatar_url(&mut self, avatar_url: String) -> &mut Self {
        self.avatar_url = Some(avatar_url);
        self
    }

    pub fn tts(&mut self, tts: bool) -> &mut Self {
        self.tts = Some(tts);
        self
    }
}

pub enum ExecuteContent {
    Content(Content),
    File(File),
    Embeds(Vec<Embeds>),
}

#[derive(Serialize,Debug)]
pub struct Content(pub String);

impl Content {
    pub fn new(content: String) -> Content {
        Content(content)
    }
}

#[derive(Serialize,Debug)]
pub struct File();

#[derive(Serialize,Debug)]
pub struct Embeds();
