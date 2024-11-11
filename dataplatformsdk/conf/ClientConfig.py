from dataplatformsdk.client.credentials import CredentialProvider


class ClientConfig:
    """Configuration object for the client

        Args:
            client_name (str): A user-defined name for the client. Used to identify number of unique
            applications/scripts running on top of your data platform.
            project (str): Data Platform Project name.
            credentials (CredentialProvider): Credentials. e.g. Token, ClientCredentials.
            api_subversion (str | None): API subversion
            base_url (str | None): Base url to send requests to. Defaults to "https://api-{project}.intellistream.ai"
            headers (dict[str, str] | None): Additional headers to add to all requests.
            timeout (int | None): Timeout on requests sent to the api. Defaults to 30 seconds.
            file_transfer_timeout (int | None): Timeout on file upload/download requests. Defaults to 600 seconds.
            debug (bool): Configures logger to log extra request details to stderr.
        """

    def __init__(self,
                 client_name:str = None,
                 project:str = None,
                 credientials: CredentialProvider = None,
    ):
        pass
