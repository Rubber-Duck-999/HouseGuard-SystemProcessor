'''
Created on 10 Oct 2019

@author: simon
'''

#!/usr/bin/env python
import pika
import sys, time, json
import subprocess
###
# Network Access Controller Simulator Interface
# This is to show how the NAC could manager
# its necessary pub & sub topics with rabbitmq

### Setup of EVM connection
print("## Beginning SYPSIM")
credentials = pika.PlainCredentials('guest', 'password')
connection = pika.BlockingConnection(pika.ConnectionParameters('localhost', 5672, '/', credentials))
channel = connection.channel()
channel.exchange_declare(exchange='topics', exchange_type='topic', durable=True)
key_publish = 'Request.Power'
key_issue = 'Issue.Notice'
key_failure = 'Failure.Component'
key_event = 'Event.SYP'
#

# Publishing
result = channel.queue_declare('', exclusive=False, durable=True)
queue_name = result.method.queue
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key_event)
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key_issue)
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key_failure)
text = '{ "power":"shutdown", "severity":5, "component": "DBM" }'
channel.basic_publish(exchange='topics', routing_key=key_publish, body=text)
print("Waiting for Messages")


def callback(ch, method, properties, body):
    print(" [x] %r:%r" % (method.routing_key, body))
    time.sleep(5)
    text = '{ "power":"shutdown", "severity":5, "component": "SYP" }'
    channel.basic_publish(exchange='topics', routing_key=key_publish, body=text)

try:
    subprocess.call(["./target/debug/exeSystemProcessor"])
    channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=False)
    channel.start_consuming()
except subprocess.CalledProcessError as e:
    print(e.output)


