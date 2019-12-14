'''
Created on 14 Dec 2019

@author: Rubber-Duck-999
'''

#!/usr/bin/env python
import pika
import sys, time, json

###
# Fault Handler Integration Interface
# This is to show how the FH could manager
# its necessary pub & sub topics with rabbitmq

### Setup of EVM connection
print("## Beginning SYPSIM")
connection = pika.BlockingConnection(pika.ConnectionParameters(host='localhost'))
channel = connection.channel()
channel.exchange_declare(exchange='topics', exchange_type='topic', durable=True)
key_failure = 'Failure.Camera'
key_network = 'Failure.Network'
key_motion  = 'Motion.Detected'
key_issue   = 'Issue.Notice'
key_monitor = 'Monitor.State'
key_event   = 'Event.FH'
key_power   = 'Request.Power'
#

# Publishing
result = channel.queue_declare('', exclusive=False, durable=True)
queue_name = result.method.queue
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key_event)
channel.queue_bind(exchange='topics', queue=queue_name, routing_key=key_power)
print("Publishing to FH")

channel.basic_publish(exchange='topics', routing_key=key_failure, body="{ 'time': 12:00:34, 'type': 'Camera', 'severity': 3 }")
channel.basic_publish(exchange='topics', routing_key=key_network, body="{ 'time': 12:00:34, 'type': 'Network', 'severity': 6 }")

def callback(ch, method, properties, body):
    print(" [x] %r:%r" % (method.routing_key, body))

channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=False)
channel.start_consuming()
