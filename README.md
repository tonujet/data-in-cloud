# ia11-vorobey-ant

## lab1

 - [x] Soft delete
 - [x] Unit tests
 - [x] Integration tests
 
### Endpoint: /apiV1/repos
- Create(POST): _ 
- Update(PUT): /{id}
- List(GET): _
- Get(GET): /{id}
- Delete(DELETE): /{id}


To run server use ``` sudo docker compose up ```. This option will run server image from remote registry. The reason why I did so is because rust compilation and linking can last up to ten minutes. If you still want to build rust server localy you also can switch to [docker-compose.yaml.local](lab1/docker-compose.yaml.local)


In rust, tests are already built in language. Project include integration test ``` cargo test ``` and unit test ```cargo test -p ```. There isn't created image for testing purposed but it can be easily done