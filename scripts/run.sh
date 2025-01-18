RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

# Function for logging
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# Function for checking dependencies
check_dependency() {
    if ! command -v $1 &> /dev/null; then
        echo -e "${RED}Error: $1 is not installed${NC}"
        exit 1
    fi
}

# Check dependencies
log "Checking dependencies..."
check_dependency "rustc"
check_dependency "cargo"

# Create logs directory
mkdir -p logs

# Check if .env exists
if [ ! -f .env ]; then
    log "Error: .env file not found"
    if [ -f .env.example ]; then
        log "Creating .env from example..."
        cp .env.example .env
    else
        exit 1
    fi
fi

# Backup database if exists
if [ -f database.sqlite ]; then
    log "Creating database backup..."
    cp database.sqlite "logs/backup_$(date '+%Y%m%d').sqlite"
fi

# Build the project
log "Building project..."
if ! cargo build --release > "logs/build_$(date '+%Y%m%d').log" 2>&1; then
    echo -e "${RED}Build failed. Check logs for details.${NC}"
    exit 1
fi

# Run the application
log "Running application..."
if ! cargo run --release > "logs/run_$(date '+%Y%m%d').log" 2>&1; then
    echo -e "${RED}Application failed. Check logs for details.${NC}"
    exit 1
fi

log "Service completed successfully"