ECE1724 Project Proposal

Skyler Zhang - 1004715953

Xiao Hu - 1005684207

**Motivation**

A distributed file storage system is a network of multiple machines that store and manage data collaboratively. This architecture is commonly used in large-scale services like Amazon and Google where massive amounts of data need to be accessible. For example, Amazon uses Dynamo to support its global users with high availability so that their online ordering system is accessible at all times. Low latency is also critical—Amazon found that a 100ms delay could reduce sales by 1%, and Google saw that a 0.5-second lag in search results caused a 20% drop in traffic. To meet such demands, distributed file storage systems must prioritize scalability, availability, and low latency, allowing data to remain accessible and reliable even in the event of node crashes.

Rust is an ideal language for building distributed systems due to its focus on predictable performance, memory safety, and concurrency management. First, Rust’s lack of garbage collection eliminates runtime interruptions which are critical for latency-sensitive distributed systems. Unlike Java, which uses garbage collection that can trigger at any time, causing unexpected events that halt execution to reclaim memory. C and C++ rely on manual memory management, prone to errors like memory leaks.  Rust’s compile-time memory safety ensures consistently high performance without compromising reliability. 

Second, Rust’s borrow checker ensures memory safety by setting clear rules for how data can be accessed and modified. Rust allows only one mutable or multiple immutable references to data at all times which prevents data race and unexpected change to data. Other programming languages like C and C++ lack built-in safety controls which makes them vulnerable to issues like buffer overflows. 

Last but not least, Rust’s asynchronous I/O enhances its suitability for distributed systems by allowing multiple tasks to run simultaneously without blocking operations, a key requirement in distributed environments. Unlike Java, C, and C++, which use synchronous I/O by default, Rust’s asynchronous I/O enables concurrent task handling, improving system latency and responsiveness.

Therefore, we propose implementing a distributed file storage system that leverages the strengths of Rust programming language. To achieve scalability, availability, safety, fault-tolerance, and user-friendliness, we propose several key features: a peer-to-peer (P2P) networking protocol to enable smooth scaling as new nodes join or leave without centralized coordination. We will also combine consistent hashing with replication, and file chunking and distribution to ensure high availability and fault tolerance. For data security, we will include user authentication and access control. Lastly, a user-friendly command-line interface will ensure smooth interaction with the system. This system will leverage Rust’s strengths to create a scalable and reliable solution for file data storage and management across multiple nodes. 

\


**Objective and Key Features**

The key objective is to build a distributed file storage system in Rust that focuses on scalability, availability, safety, fault-tolerance and user-friendliness. 

**Objective1: Scalability**

Scalability emphasizes the ability of our system to handle increased workload, such as adding nodes, without compromising performance. Scalability is crucial as it allows a distributed system to meet growing demands. As user numbers and data volumes increase, a scalable system can expand its capacity without significant reconfiguration and overhead, facilitating a better user experience.

To achieve scalability, we will implement a **Peer-to-Peer network**. This feature allows for horizontal scaling, where nodes can be easily added or removed. Each node in a P2P network acts both as a client and a server, sharing resources and responsibilities.

We will use Rust's libp2p crate to build the P2P architecture. libp2p provides protocols for node discovery, communication, and data transfer. Nodes can discover each other using decentralized mechanisms like the Kademlia Distributed Hash Table (DHT). This decentralized architecture allows the system to scale horizontally without a centralized bottleneck.

**Objective2: Availability**

Availability is the system's ability to keep operational to users at all times. In distributed systems, high availability ensures that services continue to operate, even if some nodes fail. High availability is the key to providing reliable services. It builds user trust and prevents disruptions that could lead to data losses or inaccessibility. 

The **Peer-to-Peer network** also ensures high availability. By distributing services across multiple nodes and eliminating a central server, the system eliminates the single point of failure. If some nodes fail, others can continue to provide services, ensuring continuous operation.

Using libp2p, we will create a network where nodes communicate to each other and share responsibilities. We will implement data redundancy by replicating file chunks across multiple nodes. Health checks and monitoring mechanisms will be implemented to detect node failures, and the system will redistribute data to maintain redundancy and availability.

**Objective3: Safety**

Safety refers to the ability of protecting the system's data and resources from unauthorized access. It enforces that only authorized users can access certain parts of the system. Ensuring safety is critical to prevent confidential data leakage, unauthorized data manipulation, and other security threats, thus achieving data integrity and confidentiality. This will boost user trust in the system's security and data privacy.

To ensure safety, we need robust **User Authentication and Access Control** mechanisms. This feature verifies user identities and enforces permissions, allowing only authorized access to data and system functions.

We'll use Rust's SQLx crate to securely store user credentials and permissions in the database. Passwords will be hashed with algorithms like bcrypt, ensuring they remain protected even if the database is compromised. For authentication, we'll verify user credentials during login and issue JSON Web Tokens using the jsonwebtoken crate for session management. Access control will be implemented using role-based access control (RBAC), associating permissions with user roles. This ensures users can only perform actions and access data permitted by their roles.

**Objective 4: Fault Tolerance**

Fault tolerance is a key objective in distributed systems to ensure that data remains accessible, even in the event of node failures. To achieve this, our system uses consistent hashing with replication. Both data items and nodes are assigned positions on a circular hash ring using a hash function such as MD5. Each data item is assigned to the first node encountered clockwise on the ring. When a new node joins the network, only the data closest to the new node on the ring needs to be moved to the new node. This makes sure minimal effort is required when nodes join or leave the distributed network. 

Moreover, in order to make sure the data is accessible during node crashes, instead of storing each data item on just one node, the system will store it on 3 nodes. A file will be stored on the node it hashes to and then replicated on the next two nodes in a clockwise direction on the hash ring. If one node fails, the data is still accessible from its replicas on other nodes. Replication also helps with load distribution since requests for the same data can be served by multiple nodes, balancing the network load. 

For large files, we will divide them into smaller chunks and store them across multiple nodes. This allows parallel file handling and improves fault tolerance, as missing chunks can be recovered from replicated copies on other nodes. Together, consistent hashing with replication and file chunking allow for efficient data distribution, scalability, and fault tolerance. 

**Objective 5: User-friendliness**

In a distributed file storage system, users expect a straightforward method to interact with the system effortlessly, regardless of the underlying network complexity. To meet this expectation, we will design a front-end command-line interface (CLI) that is both accessible and intuitive. This CLI will have essential file operations: connecting to a server, uploading, and downloading files across distributed nodes. By focusing on a simplified command-line utility with three core operations, we aim to make interactions seamless and user-friendly, ensuring that users can manage files smoothly in a distributed environment.

**Tentative Plan**

To achieve our project objectives, our two-member team will strategically divide responsibilities, leverage Rust’s ecosystem, and maintain close collaboration to ensure progress. Our approach focuses on parallel development of the system components and seamless integration, ensuring all previously mentioned features are addressed effectively.

The responsibilities for each team member is defined as follows:

**Team Member 1: Backend and Networking**

Responsibilities:

1. Peer-to-Peer Networking:

   - Utilize Rust’s libp2p crate to establish the P2P architecture. This includes setting up node discovery with the Kademlia Distributed Hash Table (DHT) and managing protocols for node communication.

   - Implement consistent hashing to distribute data evenly across nodes and ensure the network can scale horizontally. 

2. Data Storage and Replication:

   - Develop the hashing mechanism (e.g. MD5) to map data to nodes on the hash ring.

   - Ensure data redundancy by replicating each file chunk across several nodes. This enhances fault tolerance and maintains data availability even if nodes fail.

   - Implement algorithms to divide large files into smaller chunks, facilitating efficient storage and parallel processing.

3. File Chunking and Distribution:

   - Divide files into chunks and store each chunk across different nodes

   - Ensure each chunk is replicated 2 times so that it can be recovered even if the primary replica fails. 

   - Implement file distribution hashmap for efficient chunk location tracking and accessing. 

Core tasks:

- Set up the P2P network using _libp2p_.

- Implement consistent hashing and data replication mechanisms.

- Develop file chunking and distribution logic.


### **Team Member 2: Security and Frontend**

Responsibilities:

1. Security and Authentication:

   - Use Rust’s _SQLx_ crate to securely store user credentials. Implement password hashing with bcrypt to protect user data.

   - Develop role-based access control (RBAC) using the jsonwebtoken crate to manage JSON Web Tokens, ensuring only authorized users can access or modify data.

2. Command-Line Interface (CLI):

   - Create an intuitive CLI for users to connect to the network, upload, and download files. Ensure commands are straightforward and provide clear feedback.

   - Connect the CLI with backend services, handling network communication, data requests, and security protocols effectively.

3. Fault Tolerance:

   - Develop health checks to monitor node status and detect failures promptly.

   - Automate the redistribution of data when nodes fail or new nodes join.

Core tasks:

- Implement user authentication and RBAC.

- Develop the CLI for user interactions.

- Create monitoring tools for fault detection and data redistribution.


### **Collaboration Strategy**

To ensure cohesive development, we will adopt the following strategies:

- Weekly updates: Hold brief weekly meetings to discuss progress, address challenges, and synchronize efforts.

- Version Control: Use Git for code management, enabling seamless collaboration, and version tracking.

- Shared Documentation: Maintain documentation outlining design architecture and usage guidelines to facilitate knowledge sharing.

- Continuous Integration: Set up a testing pipeline, ensuring the system remains robust and has no integration issues.


### **Time Management**

Given the short project duration, we will prioritize the core features while assign lower weight to other features:

- Core features: The P2P network, including consistent hashing, and basic data storage and replication. The SQLx database setup and interaction. The security mechanisms, including authentication and access control. 

- Other features: The CLI, including ease to use features. Fault tolerance, including health check and redistribution mechanisms.


### **Testing and Quality Assurance**

In the final weeks, we will conduct integration testing to ensure all modules interact correctly. We will conduct stress tests to evaluate scalability and fault tolerance by simulating node failures and high-load conditions.

In conclusion, by clearly dividing responsibilities and maintaining close collaboration, our two-member team is ready to develop a functional and reliable distributed file storage system in Rust within a few weeks. Leveraging Rust’s powerful libraries and focusing on core functionalities ensures that we deliver the final product timely and meet our dedicated project objectives efficiently and effectively.
