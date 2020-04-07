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
key_failure = 'Failure.Component'
key_event = 'Event.SYP'
#

# Publishing
result = channel.queue_declare('', exclusive=False, durable=True)
queue_name = result.method.queue
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key_event)
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key_failure)
text = '{ "power":"shutdown", "severity":5, "component": "DBM" }'
channel.basic_publish(exchange='topics', routing_key=key_publish, body=text)
text = '{ "power":"shutdown", "severity":5, "component": "FH" }'
channel.basic_publish(exchange='topics', routing_key=key_publish, body=text)
text = '{ "power":"shutdown", "severity":5, "component": "EVM" }'
channel.basic_publish(exchange='topics', routing_key=key_publish, body=text)
print("Waiting for Messages")
count = 0
queue_empty = False

def callback(ch, method, properties, body):
    print(" Received %r:%r" % (method.routing_key, body))
    print("Count is : ", count)
    time.sleep(0.3)
    text = '{ "power":"shutdown", "severity":5, "component": "SYP" }'
    if count == 2:
        print("Publishing Message")
        channel.basic_publish(exchange='topics', routing_key=key_publish, body=text)
        queue_empty = True



while not queue_empty:
    method, properties, body = channel.basic_get(queue=queue_name, auto_ack=False)
    if not body is None:
        callback(channel, method, properties, body)
        count = count + 1
       
    #try:
    #    subprocess.run(["./target/debug/exeSystemProcessor"])
    #except subprocess.CalledProcessError as e:
    #    print(e.output)
