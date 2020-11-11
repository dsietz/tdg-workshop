# Starting Kafka

At this point, Kafka has already been installed and all we have to do is start the service.

## Step 1

Let first start the Zookeeper service. In a new terminal \(Window &gt; New terminal or `Alt-T`\) run the following script.

```text
./scripts/zookeeper-start.sh
```

## Step 2

Next, let's start the Kafka service. In a new terminal \(Window &gt; New terminal or `Alt-T`\) run the following script.

```text
./scripts/kafka-start.sh
```

> Tip: You can stop the services using the following commands:
>
> ```text
> ./scripts/kafka-stop.sh
> ```
>
> ```text
> ./scripts/zookeeper-stop.sh
> ```

## Step 3

Let's try out Kafka. We will have a producer to send messages to a topic that a consumer will be reading.

**Producer** In a new terminal \(Window &gt; New terminal or `Alt-T`\) run the following command:

> Ignore any warnings

```text
kafka_2.13-2.6.0/bin/kafka-console-producer.sh --topic quickstart-events --bootstrap-server localhost:9092
```

**Consumer** In a new terminal \(Window &gt; New terminal or `Alt-T`\) run the following command:

> Ignore any warnings

```text
kafka_2.13-2.6.0/bin/kafka-console-consumer.sh --topic quickstart-events --from-beginning --bootstrap-server localhost:9092
```

We are now ready to broker messages. In the **Producer** terminal, type some text and press `enter`. In the **Consumer** terminal you should see your text getting read.

