# ia11-vorobey-ant

## lab1

- [X]  Soft delete
- [X]  Pagination
- [X]  Unit tests
- [X]  Integration tests

### Endpoint: ```/api/v1/repos```

- Create(POST): ```_```
- Update(PUT): ```_/{id}```
- Delete(DELETE): ```_/{id}```
- Get(GET): ```_/{id}```
- List(GET): ```_?take={}&offset={}```

To run server use ```sudo docker compose up```. This option will run server image from remote registry. The reason why I did so is because rust compilation and linking can last up to ten minutes. If you still want to build rust server locally you also can switch to [docker-compose.local.yaml](docker-compose.local.yaml)

In rust, tests are already built in language. Project include integration test ```cargo test``` and unit test ```cargo test -p repo```. There isn't created image for testing purposed but it can be easily done

## lab2

- [X]  Soft delete
- [X]  Pagination
- [X]  Unit tests
- [X]  Integration tests

### Endpoint: ```/api/v1/users```

- Create(POST): ```_```
- Update(PUT): ```_/{id}```
- Delete(DELETE): ```_/{id}```
- Get(GET): ```_/{id}```
- List(GET): ```_?take={}&offset={}```

During this lab I used mongodb as noSQL database

To run both the server and the tests, follow the instructions from the first lab. To mock the DB, I decided to write my own in-memory implementation of the most popular mongo functions. I chose this hard approach because there is no suitable library that simulates this behavior. I was forced to do it

## lab3

- [X]  User to repo blob connection(1y:My)
- [X]  Pagination
- [X]  Unit tests
- [X]  Integration tests

### Endpoint: ```/api/v1/users/{user_id}/repos```

- Add pair(POST): ```_/{repo_id}```
- List pair(GET): ```_?take={}&offset={}```
- Delete pair(DELETE): ```_/{repo_id}```

To connect user collection from MongoDb and repository table from Postgresql as 1y:My I used aws S3bucket

To run both the server and the tests, follow the instructions from the first lab. For testing, InMemory object storage that can reproduce the behavior of such cloud object storage as S3Bucket or Azure Blob Storage


## lab4

- [X]  User repo info connection(My:My)
- [X]  Pagination
- [X]  Unit tests
- [X]  Integration tests

### Endpoint: ```/api/v1/user-repo-infos```

- Get info(POST): ```_/{user-repo-info_id}```
- Get paginated info(GET): ```_?take={}&offset={}```

### Endpoint: ```/api/v1/users``
- Get repo connection info for user: ```_/{user_id}/repo-infos```


As you can see above, there aren't any delete, create, or update endpoints. The reason why I decided to do this is that this entity is created automatically and, obviously, it can't be updated and deleted because this is system information(at least directly)

If you want to create information about user-repo connection, you must either create a new pair or delete the old one. Afterward, the appropriate info record will be saved to MongoDB

### Endpoint: ```/api/v1/users/{user_id}/repos```

- Add pair(POST): ```_/{repo_id}```
- Delete pair(DELETE): ```_/{repo_id}```

As a message broker was used RabbitMq that supports AMQP. The needed driver was used to access the message broker using this protocol


## lab5
- [X] GraphQL endpoint for lab1 lab2 lab4 services

- [ ] Docker is unavailable for this lab

### Endpoint and web interface: ```/api/v1/graphql```
Similar GraphQL queries and mutation were added to corresponding controllers from lab1, lab2, and lab4. Since I am using controller - service - repository architecture, it was a pretty easy task. The reason why I avoided lab3 is because my AWS account expired and thus occurred to my keys, so right now I technically can't use this particular endpoint. It is true, that I could recreate the account and add a new configuration to the project, but I am too lazy, and the point of this lab is to learn how to use GraphQL only


## lab6
- [X] GraphQL endpoint for services from all labs
- [X] OpenAPI specification generation for all endpoints

- [ ] Docker is unavailable for this lab

### Swagger endpoint: ```/swagger-ui```
### OpenAPI endpoint: ```/api/v1/openapi.json```
GraphQL was completed but I temporarily moved from AWS to a local solution thanks to the ```object_store``` library. All supplementary structs for GraphQL and new OpenAPI implementation were deleted and I put all restrictions to the original ones. This forced me to tweak the code a bit, but that wasn't a problem. To all REST API endpoints, Swagger was added

Finally, at the end of the lab I decided to move all dtos from repos crate to the dtos crate. After a while, it turned out to me that it is the most convenient to keep all structures dedicated only for data descriptions and transactions in a separate crate
