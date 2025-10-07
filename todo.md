# Fill out change log

Migrations först: lägg upp.sql och down.sql för customers, purchase_orders,
emails_inbound, invoices_raw samt system_logs. Använd Diesels migrations och håll dem reversibla.

Generera schema.rs, sedan modeller: Customer, PurchaseOrder, EmailInbound, InvoiceRaw.
Deriva Queryable och Insertable.

Repositories per tabell: bygg små, testbara metoder med
diesel_async::AsyncPgConnection för find, upsert och list.

Pipelinekontrakt: i EventPayload definiera
UpsertCustomers(Vec<Customer>) och UpsertPurchaseOrders(Vec<PurchaseOrder>).
Ingestor gör validering och dedup på external_id. DBWriter kör batch upsert i transaktion.

Concurrency: använd tokio::select!
över broadcast för CoreEvent och mpsc för wiring. Håll tydliga gränser per kanal.

Rocket integration: request guards för auth
som i ditt exempel och managed state för pooler.

Testflöde: kör migrations i test, seed 2 kunder och 2 orders,
trigga Fetcher till Ingestor till DBWriter och asserta rader i DB.
(Se även våra mock-endpoints för Customers och PurchaseOrders som referens.)

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
