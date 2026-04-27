#!/bin/bash

COMMAND=${1:-"up"}

case $COMMAND in
    up)
        echo "Running migrations..."
        sqlx migrate run
        ;;
    down)
        echo "Reverting last migration..."
        sqlx migrate revert
        ;;
    reset)
        echo "Resetting database..."
        sqlx database reset
        ;;
    *)
        echo "Usage: ./migrate.sh [up|down|reset]"
        exit 1
        ;;
esac