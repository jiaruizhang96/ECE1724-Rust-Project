<!--StartFragment-->
## Distributed File Storage System in Rust

Skyler Zhang - 1004715953 -  Xiao Hu - 1005684207 - hx.hu@mail.utoronto.ca
<!--StartFragment-->

### **Motivation and Project Background**<a id="h.17u30qpcljpx"></a>

**Distributed File Storage Systems: Context and Challenges**

In the contemporary technological world, data storage and management have become increasingly complex, demanding solutions that can handle massive amounts of information with reliability, scalability, and performance. Distributed file storage systems is a critical technological approach to these challenges, allowing for organizations and services to manage data across multiple connected nodes efficiently.

The motivation for developing a distributed file storage system stems from real-world industry experiences that demonstrated the importance of robust, scalable, and performant data management infrastructure. Companies like Amazon and Google have demonstrated that the performance of distributed systems directly impacts user experience and, consequently, business outcomes. For instance, Amazon's research showed that even a 100-millisecond delay in page load time could potentially reduce sales by 1%, while Google discovered that a 0.5-second lag in search results could lead to a 20% drop in user traffic.

These findings illustrate the importance of designing distributed systems that prioritize four fundamental characteristics: scalability, availability, safety and low latency. Modern storage services require infrastructure that can expand to accommodate growing data demands, maintain continuous operation even when individual nodes fail, and provide rapid access to data.

**Technological Landscape**

Traditional centralized storage systems face significant limitations in meeting these requirements. As data volumes grow exponentially and user expectations for rapid access increase, centralized architectures become the bottlenecks. Centralized systems create single points of failure, are challenging to scale horizontally, and often fail to maintain consistent performance under increased load.
Distributed file storage systems emerge as a handy solution to these challenges. By distributing data and computational overheads across multiple nodes, these systems can:
   - Achieve horizontal scalability
   - Improve overall system resilience
   - Provide more consistent performance with increased load

However, implementing such systems is non-trivial. Developers must address complex technical challenges, including:
   - Efficient data distribution and partitioning
   - Maintaining data consistency across nodes
   - Ensuring secure and controlled data access
   - Managing network communication and node discovery
   - Handling concurrent read and write operations

**Rust as a Strategic Language Choice**

The selection of Rust as the implementation language for this distributed file storage system was strategic. Rust offers unique characteristics that fits perfectly with the requirements of distributed systems development:

**Performance**

Unlike Java, which uses garbage collection that can trigger at any time, causing unexpected events that halt execution to reclaim memory. Rust provides compile-time memory safety without compromising performance. The absence of a garbage collection mechanism ensures high performance, a critical requirement for distributed systems.

**Memory Safety**

Rust's borrow checker is a revolutionary approach to memory management. By enforcing strict rules about data access at compile-time, Rust eliminates entire classes of memory-related errors which is common in languages like C and C++. This includes preventing data races, buffer overflows, and null or dangling pointer dereferencing. The borrow checker ensures that at any given time, a piece of data can have either:
   - One mutable reference
   - Multiple immutable references
This approach guarantees memory safety without runtime overhead, a significant advantage in systems requiring efficient resource utilization.

**Concurrency and Asynchronous I/O**

Modern distributed systems often require concurrency operations. Rust's asynchronous I/O capabilities enable efficient, non-blocking task execution. Unlike traditional synchronous I/O models, Rust allows multiple tasks to run simultaneously without blocking operations, which is crucial for maintaining high system throughput. Moreover, the language's ownership and borrowing rules extend to concurrent programming, providing compile-time guarantees that prevent common dreaded multithreading runtime bugs such as race conditions and deadlocks.

Based on these discussions, we propose implementing a distributed file storage system that leverages the strengths of Rust programming language. To achieve scalability, availability, safety, and user-friendliness, we proposed several key features: a peer-to-peer (P2P) networking protocol to enable smooth scaling as new nodes join or leave without centralized coordination. We also leveraged file chunking and distribution to ensure high availability and fault tolerance. For data security, we included user authentication and access control. Lastly, a user-friendly command-line interface ensures smooth interaction with the system. Our system leverages Rust’s strengths to create a scalable and reliable solution for file data storage and management across multiple nodes.


### **Objective and Key Features**<a id="h.17u30qpcljpx"></a>

**Comprehensive System Design Objectives**

When developing a distributed file storage system, it's essential to establish clear and well defined objectives that address the various challenges in the distributed system design. For our project, we identified five primary objectives fundamental to our system: Scalability, Availability, Safety, Fault Tolerance, and User-Friendliness. Each of these objectives tackles a critical aspect of the distributed systems, ensuring that the final product is robust, reliable, and user-friendly.

**Objective1, Scalability: Building a Flexible and Adaptive Infrastructure**

Scalability refers to the system's ability to handle growth, in the number of users, the volume of data, or the complexity of operations, without sacrificing performance or efficiency. In our distributed file storage system, scalability is achieved through a peer-to-peer (P2P) network architecture that allows the system to expand horizontally. This means that as demand increases, new nodes can be added to the network, enhancing the system's capacity without overwhelming existing resources.

**Key Features for Scalability:**

   - Decentralized Node Discovery Using Kademlia Distributed Hash Table (DHT): To ensure that our system can efficiently locate and communicate with any node within the network, we implemented the Kademlia DHT. This decentralized approach eliminates the need for a central directory, which allows nodes to find each other based on a structured but flexible hashing mechanism. This not only speeds up the discovery process but also improves the system's ability to scale when more nodes join the network.
   - Dynamic Node Adding and Removal: One of the challenges in distributed systems is managing the dynamic nature of node participation. Our system allows nodes to join or leave the network without requiring a central authority to manage these changes. This mechanism ensures that the system remains adaptable, making it capable of adjusting to node availability without disrupting overall operations.
   - Efficient Resource Sharing Across Nodes: Scalability is not just about adding more nodes. It's also about making the best use of current resources. Our system splits file data into chunks to facilitate efficient sharing of storage space, processing power, and bandwidth across all nodes. This approach maximizes the system's overall capacity and ensures that resources are utilized effectively, even when the network grows.


**Objective2, Availability: Ensuring Continuous Operation and Data Accessibility**

For a distributed system, high availability is crucial. It means that the system remains functional and accessible to users even when an individual node fails or network disrupts. For our distributed file storage system, achieving high availability involves eliminating the single points of failure and implementing robust redundancy mechanisms to ensure that data remains accessible at all times.

**Key features for Availability:**

   - Distributed Data Replication: To prevent data loss and ensure accessibility, our system replicates data across three nodes. By maintaining several copies of each file, the system can quickly retrieve data from a different node if the primary storage location fails. This redundancy is key to maintaining continuous access to files, regardless of individual node failures.
   - Automatic Data Redistribution When Node Fails: When a node fails or becomes unreachable, the system automatically redistributes its data to other active nodes. This process ensures that data remains available even during unexpected node failure, maintaining the system's overall reliability and user trust.
   - Health Monitoring and Node Failure Detection: Continuous health monitoring is essential for identifying and responding to node failures promptly. Our system includes mechanisms to check the status of each node, detecting failures quickly and triggering the data redistribution to maintain availability.


**Objective3, Safety: Ensuring Robust Security and Access Control**

Safety in a distributed file storage system involves the protection of both system resources and the data stored within it from unauthorized access and malicious activities. Our objective is to create a secure environment where data integrity and confidentiality are enforced, and access is controlled based on the users’ permissions.

**Key Features for Security:**

   - User Authentication Using Cryptographic Key Pairs: To verify the identity of users that try to access the system, we implemented cryptographic key pairs. Each user is assigned a unique pair of keys—one public and one private—that are used to authenticate their identity securely. This method ensures that only authorized users who have their keys registered in the system can access the system and the data.
   - Role Based Access Control: Managing who has access to what data within the system is essential for maintaining security. Our implementation uses role based access control to define roles with specific permissions, ensuring that users can only perform actions and access data that their role permits. This approach to control access helps prevent unauthorized access.
   - Credential Storage: Protecting user credentials is essential to protect the system from being compromised. We implement secure storage mechanisms for all credentials, using encryption and other measures to prevent unauthorized access or tampering with the user credential data.
   - Signature Based Request Verification: To ensure that all requests to the system are legitimate and have not been tampered with third parties or attackers, we use signature based request verification. Each request is signed with the user's private key, and the system verifies the signature using the corresponding public key stored securely. This process adds an additional layer of security, enforcing the authenticity and integrity of all operations that come from any users.
   - Fined Grained Permission Management for Data Keys: Beyond user roles, our system allows for fine grained control over who can access what specific data. The data is stored in our system typically through key value pairs, that the user refers to a specific piece of data through a unique key value assigned to that data upon the initial storage process. Permissions are managed at the level of individual data keys, which enables fine grained control over who can view, modify, or share what particular files. This level of detail in permission management enhances data security and ensures that sensitive information is only accessible to authorized users.


**Objective 4, User-Friendliness: Facilitating Intuitive System Interaction**

While the underlying architecture of our distributed file storage system is complex, we aimed at making the user experience as straightforward and intuitive as possible. We have designed a user-friendly interface that allows users to interact with the system without needing to understand the implementation details behind its operation.

**Key Features for User-Friendliness:**

   - Simple Commands for File Upload and Download: Users can easily upload and download files using straightforward commands. This simplicity lowers the barrier for our system, allowing users of all kinds of backgrounds to manage their files without frustration in using our system.
   - Clear User Registration and Authentication Process: Registering for the system and authenticating are processes that embeds within the system, without needs for external software. Users can quickly create accounts and log in securely, with clear instructions that guide them through each step. This clarity ensures that users can start using the system without unnecessary complications.
   - Complete Packaging of Distributed File Storage Handling: Although the system operates based on a distributed network, these complexities are hidden from the user completely. Whether a file is being stored across multiple nodes or being retrieved from various locations, the user experiences the same interaction with the system, without awaring the underlying processes that implemented this process.
   - Immediate Feedback on Operations: Users receive real-time feedback on their actions, such as confirmations when a file is successfully uploaded or notifications if an error occurs. This immediate feedback can improve user confidence in the system’s functioning and allows them to understand the outcome of their interactions without waiting.


**Technical Implementation For the Features**

Transforming our objectives into a functional distributed file storage system required thoughtful design and technical implementation. Below are some of the key technical achievements for our system.

   - Utilization of Libp2p for Robust P2P Networking: We leveraged libp2p, a crate designed for decentralized applications. Each node has a unique peerID as identification via the ```libp2p::identity``` module, which creates a cryptographic key pair. The node's transport layer is established using the ```development_transport ``` function from Libp2p, ensuring secure WebSocket communication.

   - Integration of Kademlia DHT for Data Stoarge: The Kademlia DHT is initialized using the Kademlia structure with an in-memory storage backend ```MemoryStore``` for key-value records. Kademlia DHT uses a binary tree structure and a XOR-based distance metric to determine the logical "closeness" of nodes so that each node only handles a subset of the entire dataset for scalability and fault-tolerance. The decentralized nature of the Kademlia DHT ensures that memory is shared among all participating nodes, with each node acting as both a storage provider and a requester.

   - Implementation of mDNS for Local Network Peer Discovery: Peer discovery is facilitated by mDNS, where the ```MdnsEvent::Discovered event``` triggers the addition of discovered peers' addresses to the Kademlia routing table. This allows nodes to locate each other without manual configuration, ensuring dynamic and decentralized peer discovery within the local network.

   - Implementation of file chunking and storage: When storing files, we split the file into 500-character chunks. Each chunk is stored as a separate key-value pair in the DHT. The total number of chunks is stored as metadata in the DHT as well. Each chunk and the metadata are also replicated to multiple peers. For retrieval, we first fetch the metadata from the DHT and then fetch each chunk sequentially based on the metadata. The file is reconstructed by writing each chunk to disk. The metadata retrieval and chunk fetching are handled by processing ```KademliaEvent::OutboundQueryCompleted``` events, which return records or errors from the DHT. The final reconstructed file is saved with the file's original key as the name. 

   - User Management with Ed25519 Cryptographic Signatures: Security is paramount in our system, and we achieve this through the use of Ed25519 cryptographic signatures. These signatures provide strong authentication and ensure that all user actions are verifiable. By using Ed25519, we balance security with performance, allowing for fast and reliable cryptographic operations without significant overhead.

   - Flexible File Storage Through Chunk-Based Distribution: Our system stores files by breaking them down into smaller chunks, which are then distributed across multiple nodes. This chunk-based approach not only enhances fault tolerance by ensuring that no single node holds an entire file but also improves scalability. Users can upload and download files, while the system manages the distribution and retrieval of chunks behind the scenes.

   - Access Control and Authentication Mechanisms: Beyond user authentication, our system incorporates detailed access control mechanisms that govern how users interact with data. By implementing role-based permissions and granular access controls for individual files, our system ensures that data is only accessible to those with the appropriate permissions.

In summary, by addressing each of these objectives and implementing the corresponding features, our distributed file storage system serves as a sound solution to data management challenges. By focusing on scalability, availability, safety, fault tolerance, and user-friendliness, we have built a system that is not only technically sound but also accessible for users.


## Reproducibility Guide

## Prerequisites

1. Install [Rust](https://www.rust-lang.org/tools/install). 
2. Clone this repository and checkout the branch kv_storage, all our implementations are in this branch. Afterwards, build the project through cargo build:
   ```bash
   git clone <git@github.com:your_github_username/ECE1724-Rust-Project.git>
   cd <ECE1724-Rust-Project>
   git checkout kv_storage
   cargo build --release
    ```
3. If you are running on a Mac machine, consider switching off the firewalls on your machine by going to setting>>Network>>Firewall advanced settings, and switch it off. We oberved that the fire wall can potentially block the multicast, causing the nodes to have trouble finding the peers.
   
## Running the Project
### Step 1: Start Nodes
1. Open four or more terminal windows. Each data is stored with three replicas, at least four servers must be running in the distributed system. If you do not run the program in at least four terminals, there will be less than four nodes online, and the uploading and retrieval in step 5 will fail with an error message complaining there are not enough peers in the network.
2. Run the program in each terminal to start nodes:
   ```bash
   cargo run
   ```
   Each node will initialize and start listening on a randomly assigned port.
### Step 2: Register the user
1. To register a user: in one of the terminals, run:
    ```bash
    register <username>
    ```
    This will register this user and creates a public-private key pairs for the user. The private key for the user will be stored locally in the directory called /private_keys under the root directory of this project. While the public key will be printed in the commandline for later usages. This design aims to keep the private key secure while using the public keys, which abides to the principles of using key pairs.
### Step 3: Get access permission for a file
1. Then, get access permission for this user on a file using the public key generated in step 2 and the key for the file to be uploaded:
    ```bash
    permission <file_key> <users_public_key>
    ```
    This will grant access permission for this user on the specific file key. This enforces access control on a granularity of single files as discussed in the features section.
### Step 4: Create the signature
1. Then, create a user signature using the username and the key for the file to be uploaded:
    ```bash
    sign <username> <file_key>
    ```
    This will generate a signature based on the user's private key on this specific file. Digital signatures enforces not only authenticity, but also non-repudiation in that the user cannot deny they have signed this file key. Both the generated signature and the public key of the user will be printed in the terminal, and these two components will be used in the next step for uploading or retrieving the file.
### Step 5: Store or retrieve the file
1. Now with the signature generated, we can store or retrieve files.
   It is recommended to use a text file, eg. ```<unique_file_name>.txt``` for testing purpose. 
   For storing. In one of the terminals, run:
    ```bash
    put -f <file_key> <absolute_path_to_file> <users_public_key> <the_signature_of_this_user_on_this_file_key>
    ```
   For retrieving. In one of the terminals, run:
    ```bash
    get -f <file_key> <users_public_key> <the_signature_of_this_user_on_this_file_key>
    ```
    The retrieved file will be written to current directory with name ```<file_key>.txt```

   Note that for retrieving this same file on a different node, you need to register another user on that node and go through steps 1-4 with this same file key and that newly registered user's name and generated public key. We decided to not propagate the user credentails from the node that registers the user to other peer nodes to enforce a distributed storage of meta data.
### Step 6: Exiting the program on one node
1. To gracefully exit the program, run:
    ```bash
    exit
    ```
    in the terminal for the node you want to exit.
<!--EndFragment-->

<!--EndFragment-->


## Contributions by each team member


**Xiao Hu implemented the following:**


The authentication functionalities, including the authentication through key pairs, the role based access control, and the digital signature.
Enforce the authenticity and check for user credentials during uploading and retrieving files.
The command line interface that supports all the operations our system supports.


**Skyler Zhang implemented the following:**


## Lessons Learned and Concluding Remarks


**Lessons Learned**


Throughout the development of our distributed file storage system, our team encountered numerous challenges that provided valuable insights and growth opportunities. One of the primary lessons was the importance of effective communication and collaboration. As we worked across different components of the system, clear and consistent communication ensured that both of us were aligned.
We also learned the significance of thorough planning and flexibility. While we had a comprehensive plan before the implementation, there were several unexpected obstacles that required us to modify our approach. This experience taught us to balance structured planning with the ability to adjust when necessary.


Additionally, we gained a deeper understanding of using Rust for building real systems.  We also learned more about distributed system concepts such as scalability, and security. Practical usage of the Rust programming language reinforced our theoretical knowledge and highlighted the technicalities involved in building robust, real-world Rust applications.


**Concluding Remarks**


In conclusion, the development of our distributed file storage system has been a rewarding journey that combined technical aspects with teamwork. We successfully achieved our primary objectives of creating a scalable, highly available, secure, and user-friendly system, demonstrating the potential of decentralized storage solutions in addressing modern data management challenges.
We are proud of the resulting system we have built and believe that our system offers a viable solution for decentralized file storage. As we move forward, the knowledge and experience gained from this project will serve as a strong foundation for addressing more complex challenges in the field of Rust programming and distributed systems.



