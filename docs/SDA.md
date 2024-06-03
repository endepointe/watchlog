# Software Design and Architecture README

## Table of Contents

1. [Introduction](#introduction)
2. [Architectural Goals and Principles](#architectural-goals-and-principles)
3. [System Overview](#system-overview)
4. [Architectural Patterns](#architectural-patterns)
5. [Component Descriptions](#component-descriptions)
6. [Web Frontend Component](#web-frontend-component)
7. [Client Component](#client-component)
8. [API Component](#api-component)
9. [Server Component](#server-component)
10. [Documentation Component](#documentation-component)
11. [Data Management](#data-managament)
12. [Client Data](#client-data)
13. [Server Data](#server-data)
14. [Interface Design](#interface-design)
15. [Considerations](#considerations)
16. [Security](#security)
17. [Performance](#performance)
18. [Maintenance and Support](#maintenance-and-support)
19. [Deployment Strategy](#deployment-strategy)
20. [Testing Strategy](#testing-strategy)
21. [Glossary](#glossary)

---

## Introduction
The project will read log data and send it to a centralized database server. 
The server will have the option to integrate with ElasticSearch for 
additional visualization and investigative functionality. 

The log collection service should take an event log, process each event 
line, and send the data to a central server. The priority is the confidentiality, 
integrity, and availability of all data flowing through this service pipeline.

<a href="#table-of-contents" style="font-size: smaller;">back to top</a>

---

## Architectural Goals and Principles
The main architectural goals and principles are security, scalability, 
abstraction, and modularity. Architectural principles include maintaining proper 
software security practices, designing modular systems, and performing 
comprehensive tests to prove functionality.

<a href="#table-of-contents" style="font-size: smaller;">back to top</a>

---

## System Overview
An open-source tool that is available to users. The application will function on 
various operating systems, which will be tested through deployment on VMS.
The centralized server is expected to run on a Linux server to receive all the 
event logs sent by user endpoints. AWS will deploy the server.

Starting at the Data Source, event lines are sent to the Data Processing and 
Monitoring module, where they are serialized and staged to be sent across the 
network. The internal and external communication will be managed through 
TLS over TCP sockets, allowing for low-latency monitoring and data feeding. 

![](log-collection-visual.png "Log Collection System")
<a href="#table-of-contents" style="font-size: smaller;">back to top</a>

---

## Architectural Pattern
We will adapt the client-server, producer-consumer, event-sourcing, and 
microservices patterns. 

### Client Server Pattern
The client-server model, where the clients on endpoints send event logs to a 
centralized server for processing and storage.

### Microservices Pattern
Using an API to handle the data access, each api endepoint is responsible 
for a specific aspect. This will make updating and maintenance easier by
isolating the frontend, server, client, and api components.

### Producer Consumer Pattern
Any component that needs to handle the input -> output of data asyncronously will
benefit from using threads to break up tasks for efficiency of time and/or
memory.

### Event Sourcing Pattern
For flexible, real-time data handling, this pattern can be beneficial to stream 
and store log data continuously.

<a href="#table-of-contents" style="font-size: smaller;">back to top</a>

---

## Component Descriptions

### Web Front-End Component
The web frontend will provide the user registration, account login, and distribution interface for the following:

- TLS certificates and their private keys
- ability to copy their OTP (important because message key is not preserved)
- software binary distribution.
- quick glance of existing log sources (not the log data itself)
- The instance IP and port connected to their account

The web frontend will also house the user documentation. See (documentation)[#documentation-component] section.

[Web Front-End Repository](https://github.com/SecurityLogMiner/log-collection-frontend)

### Client Component
The client will consist of a GUI (default) that prompts the user to provide either 
their OTP or user/pass. Once authenticated, the user will have full visibility of 
log data. The user will be able to interact with each log source, download log 
data from the central server, and add/remove log sources.

While the default frontend will be a GUI, the user has the option of running the
software from a command-line interface. 

In either case, the client frontend will use the same functionality to accomplish
the tasks.

From either the GUI or command-line interface, the user will provide
configuration file that contains a path (or paths) to the log sources, the format
of the log for each of the supplied paths, a destination IP/port, and the
certificate and private key information they were supplied from the web
interface.

[Client Repository](https://github.com/SecurityLogMiner/log-collection-client)

### API Component
This component of the product will handle all the requests from the client, and
on behalf of the client, including:
- account creation/deletion/authentication
- software distribution
- certificate download
- certificate revokation/renewal
- Single OTP message key issuance 
- user list retrieval (server only)
- instance configuration

These api endpoints must also be protected. The server component must be the only
entity that is allowed to revoke and renew certificates, issue OTPs, and retrieve a list of existing users. 

Taking actions on behalf of the user requires that the client verify themselves 
with a OTP or user/pass combination. When the endpoint receives the request, the 
corresponding database tables are updated with the new certificate and private key 
information generated by the server. The database tables are encrypted by the user's 
public key. This is important - Clients are not allowed to generate their own 
certificates or private keys. The private keys issued to the user will never be 
preserved on the server. The user will be responsible for keeping their credentials 
in a safe location.

Each client should be able to access account information by providing api
verification tokens.

Example Endpoints:
- /users
    - returns list of users (server access only)
- /revoke/user
    - revokes the certificate on behalf of the user.
- /renew/user
    - renews the certificate on behalf of the user
- /issue
    - user receives a message key to initiate their TOTP in their authenticator
      application. The message key is not preserved and the secret key is shared
      between the client authenticator app and the uuid-secret table
- /config
    - post request the passes a json object containing the items in the
      configuration file.

[API Repository](https://github.com/SecurityLogMiner/log-collection-api)

### Server Component
Considering that the central server will be a destination for many users, it is
important that this component of the project can scale in the future. For now,
during beta testing, the server should be able to create isolated instances for
each user that is generating incoming logs.

Each instance will be tied to a user using a unique key generated an existing, and validated, user. 
Each instance will store the incoming logs into a database 
instance and provide an extension to ElasticSearch should the user want that 
additional functionality.

[Server Repository](https://github.com/SecurityLogMiner/log-collection-server)

### Documentation Component
It does not have to be complicated but it must be organized. Projects die when
the documentation is either unavailable or lacks enough information to help users
(and developers) get started.

At a minimum, the user documentation should include the following:
1. Getting Started:
    - how to create an account
    - how to install software
    - how to run the client
    - configuration guidelines

2. Dashboard Information:
    - layout
    - how to view logs

<a href="#table-of-contents" style="font-size: smaller;">back to top</a>

---

## Data Management

### Client Data
Incoming log data will be defined using the configuration file located on the
client. The current format is:
```
# server address to connect to:
server_address 0.0.0.0 

# Server port to use:
server_port 54321

# Format of log file:
field_values field_1 field_2 ... field_n

# Path to TLS Credentials:
credentials path/to/creds

# Path to Private key:
pkey path/to/private/key
```

### Server Data
Each user will require an isolated database instance and each log source will be
stored in a corresponding table. 

#### user-uuid Table
The user database will contain all the users that have created an account and
will be mapped to a unique user id using a key:value structure. The key will be
encrypted and only available to the server using the server's private key. Access
Control lists should be established to protect this private key until an
improvement is proposed.

#### uuid-ip Table
Using the unique user id will identify the instance database containing the IP
and port information of all users.

#### uuid-cert Table
The unique user id and certificate info, where the certificate information is
encrypted using the CLIENT's public key and can only be decrypted using the
CLIENT's private key. The server will not have access to the data, other than
deleting the key value pair and reissuing a certificate. 

#### uuid-secret Table
Key-Value pair containing the user id and corresponding TOTP secret, ecrypted by
the server public key. This secret is not the same as the secret used with the
certificate.

<a href="#table-of-contents" style="font-size: smaller;">back to top</a>

---

## Interface Design

The interface design will consists of event logs coming from a primary source such as an endpoint. This data will be showcased and alerted and notify the system whether the log 
information contains malicious content.

The majority of the details provided will be encapsulated by the <a href="#component-descriptions" style="font-size: smaller;">component descriptions.</a>
The interface will consist of a front-end implementation using Svelte and Rust for the backend functionality.



<a href="#table-of-contents" style="font-size: smaller;">back to top</a>

---

## Considerations

### Security
Focus on: Confidentiality, Integrity, Availability

The server will generate the client's certificate and private key. This private
key will have to be randomly generated using openssl-genpkey. The client's
private key should not be stored on the server but the server will have read 
access to the user and uuid tables. If the user deletes their account, their
information should be removed from all databases and tables.


### Performance
Long-term goal is that the server is able to scale the database instances for all
users using the service. 

While system dependent, the client should be able to handle around 5K events per 
second. Read the following for Event-Per-Second:

[Event Per Second](https://content.solarwinds.com/creative/pdf/Whitepapers/estimating_log_generation_white_paper.pdf)

### Maintenance and Support
Members of the organization are able to help maintain the project, with a
handful of CODEOWNERS that help guide the updates as the project grows.

Anyone using the product will be able to submit issues that will be used to
make improvements and take suggestiongs under consideration.

<a href="#table-of-contents" style="font-size: smaller;">back to top</a>

---

# Testing Strategy
Using Postman and Rust tests, testing can be accomplished at any step in the
software development stage. First, for each feature, the contributor will:

    1. Analyze the requirements
    2. Plan appropriate tests (security, performance, regression, user)
    3. Execute those tests
    4. Use the results of the test for further discussion

<a href="#table-of-contents" style="font-size: smaller;">back to top</a>

---

## Glossary

| **Terminology**           | **Definition** |
|--------------------|--------------|
| *User Interface* |  User interactions are managed by a front-end implementation.|
| *Backend Server*  |  Processes requests, business logic, and interfaces with the database. |
| *Database* |Stores and manages data within a database for event logs in AWS. |
| *Data Producer* | Grabs a raw line from the event log |
| *Data Consumer* | Parses raw line using config format |
| *Collector* | Formatted and Serialized data is queued for Sender |
| *Sender* | Depending on central server status, the data is sent to its location|
| *Listeners* | Listen for status |
| *API* | Application Programming Interface: provides an interface for working with their specified components|
| *Central Server*| The destination of formatted event logs |
| *Offline Storage* | Data reservoir for redundancy/recovery if needed.|
| *AWS* | Centralized Server using Amazon Web Services|
| *Endpoint* | Device of a system that sends or receives data|
| *CRUD* | Create, Read, Update and Delete|
| *SSL\TLS* | Secure Sockets Layer or Transport Layer Security are protocols for encrypting data transmitted over the web. Ensures secure communication between a client and server|
|*Event Logs*| Rercods events or activities generated by a system or network|
|*Centralized Server*| Single server to collect and manage services from multiple endpoints|
|*TOTP*| Time based one-time password in RFC 6238|


<a href="#table-of-contents" style="font-size: smaller;">back to top</a>

---


