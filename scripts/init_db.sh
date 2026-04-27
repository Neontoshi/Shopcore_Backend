#!/bin/bash

# Initialize database
echo "Initializing database..."

# Run migrations
sqlx migrate run

# Seed initial data (optional)
if [ -f "scripts/seed_db.sh" ]; then
    echo "Seeding database..."
    bash scripts/seed_db.sh
fi

echo "Database initialization complete!"