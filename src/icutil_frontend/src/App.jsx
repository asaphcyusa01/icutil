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
