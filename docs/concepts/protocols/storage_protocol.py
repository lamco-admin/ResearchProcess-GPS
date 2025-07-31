"""
ResearchProcess-GPS Storage Protocol

Defines the storage abstraction layer that allows RGPS to work with
any backend: Git, PostgreSQL, JIRA, cloud services, etc.
"""

from abc import ABC, abstractmethod
from typing import Dict, List, Optional, Any, Set, Tuple
from datetime import datetime
from uuid import UUID
from enum import Enum

from .entity_protocol import EntityProtocol, TheoryProtocol, Version


class StorageCapability(Enum):
    """Capabilities that storage backends may support"""
    VERSIONING = "versioning"
    BRANCHING = "branching"
    MERGING = "merging"
    QUERYING = "querying"
    STREAMING = "streaming"
    ENCRYPTION = "encryption"
    COMPRESSION = "compression"
    TRANSACTIONS = "transactions"
    GRAPH_QUERIES = "graph_queries"


class ConflictResolution(Enum):
    """How to handle conflicts during operations"""
    FAIL = "fail"
    THEIRS = "theirs"
    OURS = "ours"
    MERGE = "merge"
    ASK = "ask"


class StorageProtocol(ABC):
    """
    Abstract storage protocol that all backends must implement.
    This enables RGPS to work with Git, databases, APIs, etc.
    """
    
    @abstractmethod
    def get_capabilities(self) -> Set[StorageCapability]:
        """Return capabilities this storage backend supports"""
        pass
    
    @abstractmethod
    def initialize(self, config: Dict[str, Any]) -> None:
        """Initialize storage with configuration"""
        pass
    
    # Entity Operations
    
    @abstractmethod
    def store_entity(self, entity: EntityProtocol, theory_id: Optional[UUID] = None) -> str:
        """
        Store an entity, returning a storage reference.
        If theory_id provided, store in that theory context.
        """
        pass
    
    @abstractmethod
    def retrieve_entity(self, reference: str, version: Optional[str] = None) -> EntityProtocol:
        """Retrieve an entity by reference, optionally at specific version"""
        pass
    
    @abstractmethod
    def update_entity(self, entity: EntityProtocol, conflict_resolution: ConflictResolution = ConflictResolution.FAIL) -> str:
        """Update an existing entity, handling conflicts as specified"""
        pass
    
    @abstractmethod
    def delete_entity(self, reference: str, soft: bool = True) -> bool:
        """Delete an entity (soft delete by default)"""
        pass
    
    @abstractmethod
    def entity_exists(self, reference: str) -> bool:
        """Check if an entity exists"""
        pass
    
    # Versioning Operations
    
    @abstractmethod
    def get_entity_history(self, reference: str) -> List[Version]:
        """Get version history for an entity"""
        pass
    
    @abstractmethod
    def create_version(self, entity: EntityProtocol, message: str) -> Version:
        """Create a new version of an entity"""
        pass
    
    @abstractmethod
    def compare_versions(self, ref1: str, version1: str, ref2: str, version2: str) -> Dict[str, Any]:
        """Compare two versions of entities"""
        pass
    
    # Theory Operations
    
    @abstractmethod
    def create_theory(self, theory: TheoryProtocol) -> str:
        """Create a new theory branch"""
        pass
    
    @abstractmethod
    def list_theories(self) -> List[Tuple[str, TheoryProtocol]]:
        """List all theories with their references"""
        pass
    
    @abstractmethod
    def switch_theory(self, theory_ref: str) -> bool:
        """Switch to a different theory context"""
        pass
    
    @abstractmethod
    def merge_theories(self, source_ref: str, target_ref: str, 
                      conflict_resolution: ConflictResolution = ConflictResolution.ASK) -> Dict[str, Any]:
        """Merge one theory into another"""
        pass
    
    # Query Operations
    
    @abstractmethod
    def query_entities(self, query: Dict[str, Any]) -> List[EntityProtocol]:
        """
        Query entities based on criteria.
        Query format is backend-specific but should support basic filters.
        """
        pass
    
    @abstractmethod
    def query_relationships(self, entity_ref: str, relationship_type: Optional[str] = None) -> List[Dict[str, Any]]:
        """Query relationships for an entity"""
        pass
    
    # Bulk Operations
    
    @abstractmethod
    def bulk_store(self, entities: List[EntityProtocol]) -> List[str]:
        """Store multiple entities efficiently"""
        pass
    
    @abstractmethod
    def bulk_retrieve(self, references: List[str]) -> List[EntityProtocol]:
        """Retrieve multiple entities efficiently"""
        pass
    
    # Transaction Support
    
    @abstractmethod
    def begin_transaction(self) -> str:
        """Begin a transaction, return transaction ID"""
        pass
    
    @abstractmethod
    def commit_transaction(self, transaction_id: str) -> bool:
        """Commit a transaction"""
        pass
    
    @abstractmethod
    def rollback_transaction(self, transaction_id: str) -> bool:
        """Rollback a transaction"""
        pass
    
    # Import/Export
    
    @abstractmethod
    def export_theory(self, theory_ref: str, format: str = "rgps") -> bytes:
        """Export a theory in specified format"""
        pass
    
    @abstractmethod
    def import_theory(self, data: bytes, format: str = "rgps") -> str:
        """Import a theory from data, return reference"""
        pass
    
    # Maintenance
    
    @abstractmethod
    def vacuum(self) -> Dict[str, Any]:
        """Clean up storage, return statistics"""
        pass
    
    @abstractmethod
    def validate(self) -> List[str]:
        """Validate storage integrity, return issues"""
        pass
    
    @abstractmethod
    def get_statistics(self) -> Dict[str, Any]:
        """Get storage statistics"""
        pass


class HybridStorageProtocol(StorageProtocol):
    """
    Special protocol for combining multiple storage backends.
    For example: Git for versions + PostgreSQL for queries.
    """
    
    @abstractmethod
    def add_backend(self, name: str, backend: StorageProtocol, 
                   capabilities: Set[StorageCapability]) -> None:
        """Add a storage backend for specific capabilities"""
        pass
    
    @abstractmethod
    def get_backend_for_capability(self, capability: StorageCapability) -> StorageProtocol:
        """Get the backend that handles a specific capability"""
        pass
    
    @abstractmethod
    def sync_backends(self) -> Dict[str, Any]:
        """Synchronize data between backends"""
        pass


class StorageAdapter(ABC):
    """
    Base class for storage adapters that implement the protocol
    for specific backends (Git, PostgreSQL, etc.)
    """
    
    @abstractmethod
    def get_protocol_implementation(self) -> StorageProtocol:
        """Return the protocol implementation"""
        pass
    
    @abstractmethod
    def validate_configuration(self, config: Dict[str, Any]) -> List[str]:
        """Validate configuration, return any issues"""
        pass
    
    @abstractmethod
    def test_connection(self) -> bool:
        """Test if the storage backend is accessible"""
        pass