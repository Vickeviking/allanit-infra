# Fill out change log

- Add models to database for seventimes, relook into their API
- Add query loogic for it
- fetch moduel now needs to continuosly try to speak with seventimes mock upon pulse
  fetch data, send it to ingestor_router(checks if its a new user? ) that validates
  and sends it to the either dead_letter which just simply logs it or to DBWrite, which adds it to the DB
- Now the data path of new entry in mock -> fetch module -> ingestor_router -> DBWrite should be complete
- Add rocket routes
  1. that fetches relevant data we store from seventimes mock in db
  2. that takes a request from client, mailtemplate + maillist + ..
     and sends it to command_bus that for now simply logs it
- Add a vue app that talks to the rocket routes, and lets you "send a mail" to one of the stored users in
  The mail list
