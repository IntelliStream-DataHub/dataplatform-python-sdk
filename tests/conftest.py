import pytest


@pytest.fixture(scope="session")
def dataplatform_client():
    from dataplatform_sdk.config import ClientConfig
    from dataplatform_sdk.clients import DataPlatformClient
    from dataplatform_sdk.credentials import TokenProvider

    my_config = ClientConfig(
        client_name="my-client",
        project="myproject",
        credentials=TokenProvider("somerandomsecrettoken"),
    )
    client = DataPlatformClient(my_config)
    return client