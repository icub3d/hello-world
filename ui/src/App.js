import React from 'react';

function App() {
  const [greeting, setGreeting] = React.useState("loading greeting...");
  React.useEffect(() => {
	let getGreeting = async () => {
	  let response = await fetch('/api/hello-world');
	  response = await response.text();
	  setGreeting(response);
	};
	getGreeting();
  }, [setGreeting]);
  return (
    <div className="App">
      <header className="App-header">
        <p>
		  {greeting}
        </p>
      </header>
    </div>
  );
}

export default App;
