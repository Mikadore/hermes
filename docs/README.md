# Hermes architecture

## Intro

Usually E2EE applications rely on storing 
data on the client's device. Since Hermes
is a web app, this isn't feasible. Instead,
the data is encrypted by client and stored on
the server, in a manner similar to [Bitwarden](https://bitwarden.com/).

## User basics

### Identity

Users are identified by their unique `username`. Every 
username is essentially an alias for the user's public *identity key*,
which together with the coresponding secret key 
form the user's *identity key pair (IKP)*.

The IKP can be used for signing data and validating the signature. 
Additionally, it is used for asynchronous key exchange. 
See [here](cryptography.md#identity-key-pair) for details.

### User Data & Authentication

Hermes needs to store highly sensitive user data on the server,
including the IKP, in order to work. This data is ***always**
encrypted on the client* before transmission.



