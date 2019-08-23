# [GSoC 2019 Report] Support Kafka Metrics in Linkerd

This page records the achievements and future work of the [Google Summer of Code 2019 project *Support Kafka Metrics in Linkerd*](https://summerofcode.withgoogle.com/projects/#5495108827348992).

Related web pages:
* [Google Summer of Code 2019](https://summerofcode.withgoogle.com/)
* [Linkerd](https://linkerd.io/)
* [Kafka Protocol](https://kafka.apache.org/protocol.html)

## Project Overview

Linkerd is a popular service mesh for Kubernetes and it can detect a few network traffic protocol automatically and create metrics accordingly. Currently, however, Kafka is not supported to be detected and it will be handled as a general TCP traffic.

This project aims to
1) Implement a Kafka codec so its protocol can be detected and decoded;
2) Integrate the codec with Linkerd;
3) Create corresponding metrics for Kafka in Linkerd;

## Achievements

In this section we will describe the achievements of the project in three parts.

### Kafka Codec

A new Kafka codec is implemented and the source code is in the [current Github repository](https://github.com/ruig2/KafkaRustCodec).

#### Design of the Codec

A major achievement of the project is the design of the codec. Since Rust is pretty different from other traditional OOP programming language such as Java, the design is also very distinctive.

The architecture of a Kafka request is in Figure 1 where the `RequestBody` type is a Rust `enum` type ([related code](https://github.com/ruig2/KafkaRustCodec/blob/980ba0d6d886fb9b1fe032e86558d9dd1c75f1a7/src/primitives.rs#L58)). Because Rust doesn't support inheritance, we use enumerate type rather than traditional OOP design. The architecture of a Kafka response is similar to a request.

<a data-flickr-embed="true"  href="https://www.flickr.com/photos/102876833@N07/48608024563/in/dateposted-public/" title="1566580196018"><img src="https://live.staticflickr.com/65535/48608024563_fd3e37c9d2_m.jpg" width="204" height="240" alt="1566580196018"></a>

Figure 1: Architecture of the Kafka

Note that each option of the `RequestBody` enumerate type is a specific Kafka request such as the metadata request ([related code](BodyMetadataRequest), and [related Kafka protocol](https://kafka.apache.org/protocol.html#The_Messages_Metadata)).
What's more, the `FromByte` trait is implement in each Kafka types. For instance, primitive types such as `i32` or more complicated types such as `HeaderRequest` all implement such a trait so that the buffer can be decoded in a smart and elegant way.
A good example is the `HeaderRequest` ([related code](https://github.com/ruig2/KafkaRustCodec/blob/980ba0d6d886fb9b1fe032e86558d9dd1c75f1a7/src/primitives.rs#L46-L55)) where you just need to call `decode_buffer(buf)` sequentially and no need to care about the type of the primitives to be decoded.

When parsing a Kafka traffic, the first a few bytes in the buffer are peeked and the request header will be decoded. If everything fine, e.g. request size is reasonable and the API key is in the protocol, then the traffic will be judged as of Kafka protocol. Later, the header and body will be analyzed accordingly, and a `DecodedRequest` or `DecodedResponse` variable will be returned to the codec caller.

### Linkerd Integration

A few PR has been made to try to integrate the above codec with Linkerd ([list of related PRs](https://github.com/linkerd/linkerd2-proxy/pulls?utf8=%E2%9C%93&q=author%3Aruig2+)).

Basically, when Linkerd is injected into Kubernetes as a proxy and a new connection is setup, the first a few bytes will be peeked to detect the protocol of the traffic.
In the above PRs, a new protocol type `Kafka` is added and it will call the codec to try to decode the traffic.
If the attempt succeeds, then the traffic will be judged as `Kafka` and the incoming traffic will be parsed as Kafka requests and outgoing one as responses.

The decoding operation is conducted in the `KafkaIo` part which is a wrapper of the original `Io` type ([related code](https://github.com/ruig2/linkerd2-proxy/blob/15a2f8840988164f08c8b8f19491e7998315cf33/src/transport/io.rs#L191)).
Inside the `read()` and `write()` method of the IO, the buffer is sent to the codec to be parsed.
In this way, we achieve the goal to analyze the traffic with a minimal change in the codebase, however, a side-effect of this design is the state of the buffer.
The buffer may be read in frame or be read a few times, which means we may parse a buffer with a broken content or decode the same buffer a few times.
To solve this issue, a state is added in the `KafkaIo` to remember which part of the buffer is decoded and which is not yet, and retry effort can be made if the content cannot be decoded meaningfully.

### Metrics Creation

Currently, the size, header and type of the request/response will be printed into the console and log when running the proxy.
More detailed metrics and more way to present the metrics are listed in the future work.

## How to run the codes

### Development Environment

To develop this project, you may need the following dependencies:

* IDE: We recommend IntelliJ or VS Code as the IDE to develop Linkerd and the Kafka codec. Both of them have powerful plugins to support Rust.

* Rust and Cargo

* Kafka: It can be run in the host machine or inside the docker or Kubernetes.

* Kubernetes: Per Linkerd suggestion, Minikube is the best option to run Kubernetes.

### Test Cases

Test cases are the best reference to start with for the project. For example:

* [Decode Kafka API version request](https://github.com/ruig2/KafkaRustCodec/blob/980ba0d6d886fb9b1fe032e86558d9dd1c75f1a7/tests/decode_requests.rs#L5)
* [Integration test to run the codec with Linkerd](https://github.com/linkerd/linkerd2-proxy/blob/15a2f8840988164f08c8b8f19491e7998315cf33/tests/transparency.rs#L80)

## Other notes
A few notes that we believe beneficial to the new developers:

### Minikube proxy for Google

Since Google is blocked in a few areas as well as the Kubernetes registry, you may need to setup a HTTPs proxy for your Minikube to download the Kubernetes dependencies.
You can setup a local HTTP proxy on the host machine such as `localhost:8123`, however, in the Minikube please use the IP address `10.0.2.2` instead of `localhost`.
This [StackOverflow page](https://stackoverflow.com/questions/9808560/why-do-we-use-10-0-2-2-to-connect-to-local-web-server-instead-of-using-computer) answers the corresponding reason.

### Linkerd cannot be compiled

One of the reason that Linkerd cannot be compiled is the NodeJS version is too old ([related Slack discussion](https://linkerd.slack.com/archives/CGR48L815/p1565182864031900?thread_ts=1565120061.030900&cid=CGR48L815)).
There is not be an error message for this and you may wait for a long long time waiting for the compile without any progress.

## Future Work

We will discuss the future work of the project in three parts, the same as the above Achievement section.

### Kafka Codec

The current version of the codec doesn't support multi-version decoding, that means, only the Kafka message of the latest version will be decoded successfully.
It can be tedious to support all the Kafka message of different versions.
To solve this problem, the official Kafka client describes the messages in the Json format ([related code](https://github.com/apache/kafka/tree/trunk/clients/src/main/resources/common/message)) and a specific generator is implemented to convert the Json files to Java codes.

It would be nice to implement a Rust generator to parse the Json files into Rust source codes.
This will make our life easier because we only need to re-run the generator when a new version of Kafka is released.

### Linkerd Integration

Although a few test cases are created, we still need more tests (both unit and integration tests) to cover more complicated scenarios before merging the PRs into the production branch.

### Metrics

A web UI will be appreciated to visualize the Kafka traffic states.
And a Prometheus report for Kafka will be wonderful.

## Acknowledge

Special thanks to the nice mentors Eliza Weisman and Thomas Rampelberg for their patience and nice instructions. I didn't know much about Rust, Kafka and Kubernetes before I join this GSoC project and I learnt a lot in this summer.

Thanks the Linkerd community for supporting.

And lastly, thanks Google for this wonderful journey.
