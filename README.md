# ia11-vorobey-ant

## lab1
To run server use ``` sudo docker compose up ```

The better way to start the server with a database is to download created image from [Hub](https://link-url-here.org). The reason is that rust compilation and linking can last up to ten minutes. What I did is create two separate images to produce the executable and the next one to prepare OS and run this executable

In rust, tests are already built in language. Project include integration test ``` cargo test ``` and unit test ```cargo test -p ```. There isn't created image for testing purposed but it can be easily done