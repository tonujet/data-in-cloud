# ia11-vorobey-ant

## lab1

- [X]  Soft delete
- [X]  Pagination
- [X]  Unit tests
- [X]  Integration tests

### Endpoint: ```/apiV1/repos```

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

### Endpoint: ```/apiV1/users```

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

### Endpoint: ```/apiV1/users/{user_id}/repo```

- Add pair(POST): ```_/{repo_id}```
- List pair(GET): ```_?take={}&offset={}```
- Delete pair(DELETE): ```_/{repo_id}```

To connect user collection from MongoDb and repository table from Postgresql as 1y:My I used aws S3bucket

To run both the server and the tests, follow the instructions from the first lab. For testing, InMemory object storage that can reproduce the behavior of such cloud object storage as S3Bucket or Azure Blob Storage
