class DataPlatformClient:
    """Entrypoint into IntelliStream DataHub Python SDK.

        All services are made available through this object.

        Args:
            config (ClientConfig | None): The configuration for this client.
        """

    _API_VERSION = "v0.1"