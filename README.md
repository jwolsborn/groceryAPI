# Simple grocery list api

## Building and running the application
**cargo build** to to build the application

**cargo run** to run the the application

The server operates on https://127.0.0.1:8000

To add an item simply make a POST request to https://127.0.0.1:8000/:item to insert an item into the grocery list

To remove an item simply make a PUT request to https://127.0.0.1:8000/:item to remove an item from the grocery list

To retrieve a list of items in the grocery list make a GET request to https://127.0.0.1:8000

## Write-up

This application was built to experiment with the actix-web crate.  I did this by creating three simple routes that allowed
the user to add and remove items as well as retieve the list of items on the list.  Since this application is being used as 
a teching exercise the list data is simply being kept in memory, thus it won't persist once the application is terminated.  To
ensure that the data was safe I used a ```Mutex``` to lock the data in the event that two routes were called at the same time.  I used the ```Responder``` trait as a return value to all my routes as this trait allowed for a variety of return values from
the route functions themselves.  More importantly it allowed me to return the data in a JSON format that is commonly used with
web APIs.  I found the whole process very similar to that of writing an Express API, so the though process was relatively the same when it came to implementation.  That being said I have had difficulty dealing with error handling, mostly due to my, inexperience. While it seems difficult to error this application I am looking into ways to add effective error handling and return meningful error responses.  I will say that creating an API in Rust is somewhat more verbose than that of other languages, however with the verbose nature of Rust comes the safety that is provided with the language.  It was reassuring to know that there was little no possibility of a data race occuring in regards to the grocery list.    
