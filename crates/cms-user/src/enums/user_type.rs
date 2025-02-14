pub enum UserTypeEnum {
    Admin,
    Member,
    Guest,
}

impl UserTypeEnum {
    pub fn as_str(&self) -> &str {
        match self {
            UserTypeEnum::Admin => "admin",
            UserTypeEnum::Member => "user",
            UserTypeEnum::Guest => "guest",
        }
    }

    pub fn to_string(&self) -> String {
        let str = self.as_str();
        str.to_string()
    }
}
