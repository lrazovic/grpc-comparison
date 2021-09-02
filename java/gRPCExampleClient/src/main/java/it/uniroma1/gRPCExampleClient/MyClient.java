package it.uniroma1.gRPCExampleClient;

import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;

import it.uniroma1.gRPCExample.HelloRequest;
import it.uniroma1.gRPCExample.HelloResponse;
import it.uniroma1.gRPCExample.HelloServiceGrpc;

public class MyClient {
    public static void main(String[] args) throws InterruptedException {
        ManagedChannel channel = ManagedChannelBuilder.forAddress("localhost", 8080).usePlaintext().build();

        HelloServiceGrpc.HelloServiceBlockingStub stub = HelloServiceGrpc.newBlockingStub(channel);

        HelloResponse helloResponse = stub
                .hello(HelloRequest.newBuilder().setFirstName("Leonardo").setLastName("Razovic").build());

        System.out.println("Response received from server:\n" + helloResponse);

        channel.shutdown();
    }
}
