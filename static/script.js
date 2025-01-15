const apiUrl = "http://localhost:6969/api/users";

// Create a new user
document
  .getElementById("create-user-form")
  .addEventListener("submit", async (event) => {
    event.preventDefault();
    const name = document.getElementById("name").value;
    const email = document.getElementById("email").value;

    try {
      const response = await fetch(apiUrl, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ name, email }),
      });

      if (!response.ok) throw new Error("Failed to create user");

      const user = await response.json();
      alert(`User ${user.name} created successfully!`);
      fetchUsers();
    } catch (error) {
      console.error(error);
      alert("An error occurred");
    }
  });

// Fetch all users
async function fetchUsers() {
  try {
    const response = await fetch(apiUrl);
    if (!response.ok) throw new Error("Failed to fetch users");

    const users = await response.json();
    const userList = document.getElementById("user-list");
    userList.innerHTML = "<h2>User List</h2>";

    users.forEach((user) => {
      const div = document.createElement("div");
      div.textContent = `Name: ${user.name}, Email: ${user.email}`;
      userList.appendChild(div);
    });
  } catch (error) {
    console.error(error);
    alert("An error occurred while fetching users");
  }
}

// Fetch users on page load
fetchUsers();
