import { useReducer, useEffect } from 'react';
import { icutil_backend } from 'declarations/icutil_backend';
import { VirtualList } from 'react-virtualized';

const initialState = {
  readings: [],
  stats: {},
  loading: false,
  error: null
};

function reducer(state, action) {
  switch (action.type) {
    case 'FETCH_START':
      return { ...state, loading: true, error: null };
    case 'FETCH_SUCCESS':
      return { ...state, loading: false, ...action.payload };
    case 'FETCH_ERROR':
      return { ...state, loading: false, error: action.payload };
    default:
      return state;
  }
}

function App() {
  const [greeting, setGreeting] = useState('');

  function handleSubmit(event) {
    event.preventDefault();
    const name = event.target.elements.name.value;
    icutil_backend.greet(name).then((greeting) => {
      setGreeting(greeting);
    });
    return false;
  }

  return (
    <main>
      <img src="/logo2.svg" alt="DFINITY logo" />
      <br />
      <br />
      <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="name">Enter your name: &nbsp;</label>
        <input id="name" alt="Name" type="text" />
        <button type="submit">Click Me!</button>
      </form>
      <section id="greeting">{greeting}</section>
    </main>
  );
}

export default App;

function ReadingList({ readings }) {
  return (
    <VirtualList
      width={800}
      height={600}
      rowCount={readings.length}
      rowHeight={40}
      rowRenderer={({ index, style }) => (
        <div style={style}>
          {readings[index].flowRate}
        </div>
      )}
    />
  );
}

// Role Management Component
const RoleManager = () => {
  const [users, setUsers] = useState([]);
  const [selectedUser, setSelectedUser] = useState('');
  const [availableRoles, setAvailableRoles] = useState(['admin', 'device_manager', 'flow_operator']);
  const [assignedRoles, setAssignedRoles] = useState([]);

  useEffect(() => {
    // Fetch users from auth backend
    auth_backend.list_users().then(setUsers);
  }, []);

  const handleRoleUpdate = async () => {
    try {
      await auth_backend.assign_roles(selectedUser, assignedRoles);
      alert('Roles updated successfully');
    } catch (error) {
      alert(`Role update failed: ${error}`);
    }
  };

  return (
    <div className="role-manager">
      <h2>Role Management</h2>
      <Select
        options={users.map(u => ({ value: u.principal, label: u.username }))}
        onChange={(e) => {
          setSelectedUser(e.value);
          auth_backend.get_roles(e.value).then(setAssignedRoles);
        }}
        placeholder="Select user"
      />
      
      <div className="role-selector">
        {availableRoles.map(role => (
          <label key={role}>
            <input
              type="checkbox"
              checked={assignedRoles.includes(role)}
              onChange={(e) => {
                const newRoles = e.target.checked
                  ? [...assignedRoles, role]
                  : assignedRoles.filter(r => r !== role);
                setAssignedRoles(newRoles);
              }}
            />
            {role}
          </label>
        ))}
      </div>

      <button onClick={handleRoleUpdate}>Save Roles</button>
    </div>
  );
};

// Add to main app component
<RoleManager />
