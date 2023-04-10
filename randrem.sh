#!/bin/bash

# parse the parameters
MINIMUM_TIME=$1
MAXIMUM_TIME=$2
shift 2
PARAMETERS=$@

while true; do
  # sleep for a random amount of time between MIN_TIME and MAX_TIME seconds
  SLEEP_TIME=$(shuf -i $MINIMUM_TIME-$MAXIMUM_TIME -n 1)
  sleep $SLEEP_TIME

  # send a notification with the parameters
  notify-send "$PARAMETERS"
done

