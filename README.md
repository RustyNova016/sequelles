# Sequelles

An almost-ORM library that empower SQL(x) lovers!

Sequelles is designed to get rid of the boilerplate SQL code that you need to deal with SQLx. 
This handles generating simple, yet efficient CRUD queries, while leaving you full control for actual queries that matter. 

Here's some highlights:
- Fully integrated on sqlx. This means you can fallback to pure sqlx anytime
- Opt-in to the ORM. Most ORMs forces you to fully declare your schema, while sequelles is usable on partiable schemas
- Minimal runtime impact. ORMs often generate queries at runtime. Sequelles generate them at compile time, making queries... *ahem* 🔥🔥🔥 Blazingly fast 🔥🔥🚀
- Transparent queries. You can inspect all the queries using macro expension. Don't like it? Remove the macro and write your own.
