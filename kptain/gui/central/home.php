<?php
// We need to use sessions, so you should always start sessions using the below code.
session_start();
// If the user is not logged in redirect to the login page...
if (!isset($_SESSION['loggedin'])) {
	header('Location: index.html');
	exit;
}
?>

<!DOCTYPE html>
<html>
	<head>
		<meta charset="utf-8">
		<link href="style.css" rel="stylesheet" type="text/css">
		<link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.7.1/css/all.css">
	</head>
	<body class="loggedin">
		<nav class="navtop">
			<div>
				<h1>K-ptain.Ratz</h1>
				<a href="home.php"><i class="icon-dashboard"></i>Dashboard</a>
				<a href="profile.php"><i class="fas fa-user-circle"></i>Profile</a>
				<a href="logout.php"><i class="fas fa-sign-out-alt"></i>Logout</a>
			</div>
		</nav>
		<div class="content">
			<h2>Central Interface</h2>
			<p>Logged as : <?=$_SESSION['name']?></p>

			<table>
				<tr>
					<th>Client details</th>
					<th>ID</th>
					<th>IP adress (Public)</th>
					<th>IP adress (Private)</th>
					<th>OS</th>
					<th>Last seen user</th>
					<th>PC Name</th>
				</tr>
				<tr>

					<td><a href="link.html">View details</a></td>
					<td><img src="/assets/france.png" /></td>
					<td>89.88.12.8</td>
					<td>192.168.90.92</td>
					<td>Windows 10</td>
					<td>amad</td>
					<td>AMADBOSS-PC</td>
				</tr>
				<tr>
					<td><a href="link.html">View details</a></td>
					<td><img src="/assets/us.png" /></td>
					<td>8.8.8.8</td>
					<td>192.168.1.10</td>
					<td>Windows 7 SP1</td>
					<td>Administrator</td>
					<td>Random-PC</td>
				</tr>
			</table>

		</div>
	</body>
</html>
