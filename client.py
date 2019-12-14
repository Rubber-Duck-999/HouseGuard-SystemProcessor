#!/usr/bin/env python
import pika
import uuid

key_failure = 'Failure.*'
key_issue   = 'Issue.Notice'
key_event   = 'Event.SYP'
key_power   = 'Request.Power'

class FibonacciRpcClient(object):

    def __init__(self):
        self.connection = pika.BlockingConnection(
            pika.ConnectionParameters(host='localhost'))

        self.channel = self.connection.channel()
        self.channel.exchange_declare(exchange='topics',
                                 exchange_type='topic',
                                 durable=True)

        result = self.channel.queue_declare(queue='',
                                            exclusive=False,
                                            durable=True)
        self.callback_queue = result.method.queue
        self.channel.queue_bind(exchange='topics',
                           queue=self.callback_queue,
                           routing_key=key_event)
        self.channel.queue_bind(exchange='topics',
                           queue=self.callback_queue,
                           routing_key=key_issue)
        self.channel.queue_bind(exchange='topics',
                           queue=self.callback_queue,
                           routing_key=key_failure)

        self.channel.basic_consume(
            queue=self.callback_queue,
            on_message_callback=self.on_response,
            auto_ack=True)

    def on_response(self, ch, method, props, body):
        print(body)
        print(method.routing_key)
        if self.corr_id == props.correlation_id:
            self.response = body

    def call(self, n):
        self.response = None
        self.corr_id = str(uuid.uuid4())
        self.channel.basic_publish(
            exchange='topics',
            routing_key=key_power,
            properties=pika.BasicProperties(
                reply_to=self.callback_queue,
                correlation_id=self.corr_id,
            ),
            body=str(n))
        while self.response is None:
            self.connection.process_data_events()
        return int(self.response)


fibonacci_rpc = FibonacciRpcClient()

print(" [x] Requesting fib(30)")
response = fibonacci_rpc.call(30)
print(" [.] Got %r" % response)
