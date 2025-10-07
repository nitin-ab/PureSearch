# PureSearch

## Introduction

PureSearch is a lightweight, Rust-based search system designed specifically for managing and querying review documents. It provides a complete solution including document storage, indexing, and a RESTful API for ingestion and search operations. The system emphasizes performance, persistence, and simplicity, making it suitable for applications dealing with user reviews, feedback, or similar text-based content.

At its core, PureSearch handles `ReviewDocument` objects, which consist of textual content and associated metadata. It supports basic CRUD operations on documents, index management, and simple search capabilities. The storage layer uses memory-mapped files for efficient persistence and recovery.

This project is structured as a Cargo workspace with multiple crates:
- `puresearch-core`: Defines core data structures and traits.
- `puresearch-api`: Implements the HTTP API using Axum.
- `puresearch-storage`: Handles persistent storage using memory-mapped segments and write-ahead logging (WAL).

## Features

- **Document Management**: Store, retrieve, and delete review documents with metadata.
- **Indexing**: Create and manage indices for organizing documents.
- **Search Functionality**: Query documents based on content (basic implementation; extendable).
- **Persistent Storage**: Uses memory-mapped files and WAL for durability and crash recovery.
- **RESTful API**: Exposed endpoints for health checks, document ingestion, retrieval, search, and index operations.
- **Efficient Operations**: Leverages Rust's performance with async I/O and efficient serialization (via bincode).
- **Testing**: Comprehensive integration tests for storage operations.
- **Modular Design**: Separate crates for core logic, API, and storage, allowing easy extension or replacement of components.

## Architecture

### Core Components

- **ReviewDocument**: A struct containing an ID (UUID), content string, metadata HashMap, and timestamp.
- **Index**: Manages collections of document IDs with metadata like name and creation time.
- **Storage Traits**:
  - `StorageEngine`: For document operations (store, get, delete, list).
  - `IndexStorage`: For index operations (store, get, list).

### Storage Layer (puresearch-storage)

- **MmapStorage**: Implements the storage traits using memory-mapped files.
- **Segments**: Data is stored in segment files for efficient access.
- **Write-Ahead Log (WAL)**: Ensures operations are durable by logging changes before committing to main storage.
- Persistence: Supports flushing changes to disk and recovering state on restart.

### API Layer (puresearch-api)

- Built with Axum for asynchronous HTTP handling.
- Endpoints:
  - `/health`: Simple health check.
  - `/documents` (POST): Ingest a new document.
  - `/documents/:id` (GET): Retrieve a document by ID.
  - `/search` (GET): Search documents with query parameters.
  - `/indices` (POST): Create a new index.
  - `/indices` (GET): List all indices.
- Responses in JSON format.

### Data Flow

1. Documents are ingested via API and stored using the storage engine.
2. Indices can be created and documents added to them.
3. Search queries retrieve relevant documents from storage, potentially filtered by indices.
4. All operations ensure persistence through the storage layer.

## Installation

### Prerequisites

- Rust (version 1.70 or later recommended).
- Cargo (comes with Rust).

### Building from Source

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/puresearch.git
   cd puresearch
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. (Optional) Run tests:
   ```
   cargo test
   ```

## Usage

### Running the API

Build and run the API server:

```
cargo run -p puresearch-api
```

The server will start on `http://localhost:3000` (configurable).

### API Examples

#### Ingest a Document

```
curl -X POST http://localhost:3000/documents \
     -H "Content-Type: application/json" \
     -d '{"content": "Great product!", "metadata": {"rating": "5"}}'
```

#### Search Documents

```
curl "http://localhost:3000/search?q=great&limit=10"
```

#### Create an Index

```
curl -X POST http://localhost:3000/indices \
     -H "Content-Type: application/json" \
     -d '"product_reviews"'
```

#### Get a Document

```
curl http://localhost:3000/documents/{document_id}
```

### Storage Configuration

The storage engine uses a directory for persistence. When initializing `MmapStorage`, provide a path:

```rust
let storage = MmapStorage::new("/path/to/storage/dir").unwrap();
```

## Testing

PureSearch includes integration tests for the storage layer. Run them with:

```
cargo test --package puresearch-storage
```

Tests cover:
- Basic storage operations (store, get, list).
- Persistence and recovery across instances.
- Delete operations.

Expand tests as needed for API and core components.

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a Pull Request.

Please ensure your code passes all tests and follows Rust idioms.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

(Note: If no LICENSE file exists, consider adding one with standard MIT terms.)
