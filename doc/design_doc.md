# Catalog

* Zilong Zhou (zilongz@andrew.cmu.edu)
* Yen-Ju Wu (yenjuw@andrew.cmu.edu)
* Chien-Yu Liu (chienyul@andrew.cmu.edu)

## Overview
>What is the goal of this project? What will this component achieve?

## Architectural Design
>Explain the input and output of the component, describe interactions and breakdown the smaller components if any. Include diagrams if appropriate.

## Design Rationale
* Correctness:
  * The catalog service ensures data consistency and correctness by adhering to the Iceberg Catalog REST interface.
  * Data durability mechanisms will be implemented to prevent data loss during restarts.
* Performance:
  * Optimization on data retrieval and storage strategies to minimize latency in metadata access.
  * Efficient indexing mechanisms, such as Bloom filters, enhance query performance to speed up metadata search operations.
  * Partitioning strategies facilitate data pruning and improve query execution performance.
* Engineering Complexity / Maintainability:
  * Centralized metadata management achieved by separating data and metadata, reducing complexity and facilitating consistent metadata handling.
  * Code modularity and clear interfaces facilitate easier updates and improvements.
* Testing:
  * Comprehensive testing plans cover correctness through unit tests and performance through long-running regression tests. Unit tests focus on individual components of the catalog service, while regression tests evaluate system-wide performance and stability.
* Other Implementations:
  * Chose the Iceberg Catalog REST interface due to its industry adoption, standardization, and compatibility with various systems.

## Testing Plan
To ensure the quality and the performance of the catalog implemented, a comprehensive testing strategy is a must. Our testing strategy will include both functional and non-functional aspects of our catalog service. 

* Functional testing
  * API tests: For functional testing, we can achieve the goal through unit tests. We will test each API endpoint implemented in our project to ensure correct behavior. We will test various input parameters and validate the response format and the status code are as expected. Also, we will try to mimic possible edge cases and errors to ensure the implementation is robust and can perform suitable error handling. By doing so, we can ensure the API works as expected and provides correct results to clients. 
  * Metadata tests: We will focus on verifying the correct storage and retrieval of metadata. Tests will include different scenarios, including some edge cases. [Quickcheck](https://github.com/BurntSushi/quickcheck) is an example for performing the testing.
  * [Documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#documentation-tests): Execute document examples 
* Non-functional testing
  * Microbenchmarking for performance evaluation: We can use [Criterion.rs](https://github.com/bheisler/criterion.rs?tab=readme-ov-file#features) and [bencher](https://github.com/bluss/bencher) to collect statistics to enable statistics-driven optimizations. In addition, we can set up a performance baseline to compare the performance with our implementation. We can measure different metrics, for example, response time, throughput, etc.  
  * Scalability test: We will try to test our implementation under increased load and ensure the correctness and efficiency at the same time.

## Trade-offs and Potential Problems
* Balancing between metadata retrieval speed and storage efficiency.
* Striking a balance between query performance and resource utilization.

## Glossary (Optional)
>If you are introducing new concepts or giving unintuitive names to components, write them down here.
