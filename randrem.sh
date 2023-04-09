#!/bin/bash

# parse the parameters
MINIMUM_TIME=$1
MAXIMUM_TIME=$2
shift 2
PARAMETERS=$@

while true; do
  notify-send "$PARAMETERS"

  SLEEP_TIME=$(shuf -i $MINIMUM_TIME-$MAXIMUM_TIME -n 1)
  echo "Sleeping for $SLEEP_TIME seconds..."
  sleep $SLEEP_TIME
done

