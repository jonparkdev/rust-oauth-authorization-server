# Rust OAuth2 Authorization Server

## Background

Every time I end up heading in to the depths of Identity and Access management, I tend to get lost... in a good way. After the haze has cleared I find myself woken up in the weeds of an RFC which has to say something. right? I must really enjoy the world of IAM. So why not get a little more familiar?

The scope of this project is layered. First, I think it's about time that I become fluent in another language. The language I have chosen is Rust. Part of this decision is due to a textbook that has found its way onto my desk. [Zero To Production In Rust](https://www.zero2prod.com/index.html?country_code=CA). Fantastic textbook so far. From what I have read so far, the knowledge in this book will not only teach me some of the basics of Rust but help me brush up on my backend engineering knowledge.

## Goals

- Build an authorization flow for PKCE grant type to interface with React App
- Incorporate some gitops to host within my home kubernetes server
  - helm chart and docker containers

## Non Goals

- OAuth2 is big... very big, so I am only looking to build one flow(at the moment).

## Plan

1) First thing that an OAuth server needs is a way to register trusted clients.
   
- POST /clients
```
// inputs payload
{
  "client_id": string
  "callback_uris": string[]
}

// outputs
{
  "client_id": string
  "client_secret": string
  "callback_uri": string
}
```

- GET /clients

- GET /clients/{client_id}

- DELETE /clients/{client_id}