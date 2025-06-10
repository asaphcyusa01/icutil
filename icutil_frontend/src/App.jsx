// Role Management Section
const [selectedUser, setSelectedUser] = useState('');
const [roles, setRoles] = useState([]);

const handleAssignRoles = async () => {
  await auth_backend.assign_roles(selectedUser, roles);
  alert('Roles updated successfully');
};

// In render():
<AdminPanel>
  <UserSelector onSelect={setSelectedUser} />
  <RoleMultiSelect value={roles} onChange={setRoles} />
  <Button onClick={handleAssignRoles}>Save Roles</Button>
</AdminPanel>