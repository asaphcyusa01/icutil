import { createContext, useContext } from 'react';
import { AuthClient } from '@dfinity/auth-client';

const AuthContext = createContext();

export function AuthProvider({ children }) {
  const [identity, setIdentity] = useState(null);

  const login = async () => {
    const authClient = await AuthClient.create();
    await authClient.login({
      identityProvider: import.meta.env.VITE_IDENTITY_PROVIDER
    });
    setIdentity(authClient.getIdentity());
  };

  return (
    <AuthContext.Provider value={{ identity, login }}>
      {children}
    </AuthContext.Provider>
  );
}