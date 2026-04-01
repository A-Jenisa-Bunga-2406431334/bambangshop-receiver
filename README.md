#### Reflection Subscriber-2

1. Yes, I explored src/lib.rs to understand how APP_CONFIG, REQWEST_CLIENT, 
and other utilities are set up for the receiver app. I learned that lib.rs 
serves as the central configuration file that initializes shared resources 
like the HTTP client and app configuration. This pattern ensures that these 
resources are only created once and shared across the application efficiently.

2. The Observer pattern makes it very easy to plug in more subscribers. 
Each new Receiver instance just needs to subscribe to the relevant product 
type, and it will automatically receive notifications. Adding more instances 
of the Main app would also be straightforward - each Main app instance 
maintains its own list of subscribers and sends notifications independently. 
The loose coupling between Publisher and Subscriber means we can scale either 
side without modifying the other's code.

3. I explored Postman's collection features to organize and test the API 
endpoints. The ability to save requests in collections and share them with 
team members is very useful for group projects. I also found the environment 
variables feature helpful for switching between different server URLs (like 
localhost:8000, 8001, 8002, 8003) without manually editing each request. 
For future projects, I would use Postman's automated testing features to 
create test suites that verify API behavior.