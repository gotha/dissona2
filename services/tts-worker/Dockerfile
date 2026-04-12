FROM python:3.12-slim-bookworm

WORKDIR /app

# Install system dependencies for audio processing
RUN apt-get update && apt-get install -y \
    ffmpeg \
    libsndfile1 \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy source code
COPY src/ ./src/

# Create models directory
RUN mkdir -p /models

WORKDIR /app/src

EXPOSE 9090

CMD ["python", "main.py"]
