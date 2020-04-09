"# jdonaldson40505" 
this application is a naive implementation of a backend for a photo storage system. It currently does not accept input so sample inputs have been provided in main under example data and input users for you to experiment with until the application becomes more functional.
in order to run simply clone and use cargo run. make sure that you have these dependancies in your cargo.toml:
lazy_static = "*"
futures = "*"
tokio = { version = "0.2", features = ["full"] }
actix-web = "2.0"
actix-rt = "1.0"
uuid= { version = "*", features = ["v4"]}
serde = {version = "*" , features = ["derive"]}
serde_json = "*"
base64 = "*"
if you want to try a different function or scenario after running the application you will need to interupt the application with "ctrl + c"
(if your on a windows machine) and the use cargo run after you've made the changes in the code. 
After running "cargo run" open your browser and go to http://localhost:8088 to see the results! 
