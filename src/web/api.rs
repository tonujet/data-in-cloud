pub enum API {
    Entity(EntityApi),
    OpenApi(OpenApi),
    GraphQL,
}

impl API {
    pub fn to_endpoint(&self) -> &str {
        use API::*;

        match self {
            Entity(api) => api.to_endpoint(),
            OpenApi(api) => api.to_endpoint(),
            GraphQL => "/graphql",
        }
    }

    pub fn to_full_endpoint(&self) -> String {
        let endpoint = self.to_endpoint();
        format!("{}{}", Self::prefix(), endpoint)
    }

    pub fn version() -> u8 {
        1
    }

    pub fn str_version() -> String {
        Self::version().to_string()
    }

    pub fn prefix() -> String {
        format!("/api/v{}", Self::version())
    }
}

pub enum OpenApi {
    Swagger,
    File,
}

impl OpenApi {
    pub fn to_endpoint(&self) -> &str {
        use OpenApi::*;

        match self {
            Swagger => "/swagger-ui",
            File => "/openapi.json"
        }
    }
}

pub enum EntityApi {
    Users,
    Repos,
    UserRepoInfos,
}

impl EntityApi {
    pub fn to_endpoint(&self) -> &str {
        use EntityApi::*;

        match self {
            Users => "/users",
            Repos => "/repos",
            UserRepoInfos => "/user-repo-infos",
        }
    }

    pub fn to_str_tag(&self) -> &str {
        use EntityApi::*;

        match self {
            Users => "Users",
            Repos => "Repositories",
            UserRepoInfos => "User repo information",
        }
    }
}

