# Catalog

* Zilong Zhou (zilongz@andrew.cmu.edu)
* Yen-Ju Wu (yenjuw@andrew.cmu.edu)
* Chien-Yu Liu (chienyul@andrew.cmu.edu)

## Overview
>What is the goal of this project? What will this component achieve?

## Architectural Design
>Explain the input and output of the component, describe interactions and breakdown the smaller components if any. Include diagrams if appropriate.

## Design Rationale
>Explain the goals of this design and how the design achieves these goals. Present alternatives considered and document why they are not chosen.

## Testing Plan
>How should the component be tested?

To ensure the quality and the performance of the catalog implemented, a comprehensive testing strategy is a must. Our testing strategy will include both functional and non-functional aspects of our catalog service. 

* Functional testing
  * API tests: For functional testing, we can achieve the goal through unit tests. We will test each API endpoint implemented in our project to ensure correct behavior. We will test various input parameters and validate the response format and the status code are as expected. Also, we will try to mimic possible edge cases and errors to ensure the implementation is robust and can perform suitable error handling. By doing so, we can ensure the API works as expected and provides correct results to clients. 
  * Metadata tests: We will focus on verifying the correct storage and retrieval of metadata. Tests will include different scenarios, including some edge cases.
  * Consistency tests: We will test the scenarios where numerous operations are operated at the same time. We will ensure that even in complicated situations, our implementation maintains data consistency with absolute correctness.
* Non-functional testing
  * Performance evaluation: We will set up a performance baseline to compare the performance of our implementation. We can measure different metrics, for example, response time, throughput, etc.
  * Scalability test: We will try to test our implementation under increased load and ensure the correctness and efficiency at the same time.



## Trade-offs and Potential Problems
>Write down any conscious trade-off you made that can be problematic in the future, or any problems discovered during the design process that remain unaddressed (technical debts).

## Glossary (Optional)
>If you are introducing new concepts or giving unintuitive names to components, write them down here.