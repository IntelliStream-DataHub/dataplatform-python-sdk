from typing import Protocol, Any

class CredentialProvider(Protocol):
    pass

class TokenProvider(CredentialProvider):
    """Token credential provider

    Args:
        token (str | Callable[[], str]): A token or a token factory.

    Examples:

            >>> from dataplatformsdk.client.credentials import TokenProvider
            >>> token_provider = TokenProvider("my secret token")

    Note:
        If you pass in a callable, we will expect that you supplied a function that may do a token refresh
        under the hood, so it will be called while holding a thread lock (threading.Lock()).
    """

    def __init__(self, token: str) -> None:
        if isinstance(token, str):
            self.__token_factory = lambda: token
        else:
            raise TypeError(f"'token' must be a string(str), not {type(token)}")

    def authorization_header(self) -> tuple[str, str]:
        return "Authorization", f"Bearer {self.__token_factory()}"

class ClientSecretProvider(CredentialProvider):

    def __init__(self, client_id: str, client_secret: str) -> None:
        self.client_id = client_id
        self.client_secret = client_secret