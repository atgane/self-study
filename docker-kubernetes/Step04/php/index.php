<html>
  <head>
    <title>
      PHP CONNECTION TEST
    </title>
  </head>

  <body>
    <?php
    $servername = "mysql";
    $database = "mysql";

    $username = getenv("MYSQL_USER"); //환경 변수로부터 유저 ID등록
    $password = getenv("MYSQL_PASSWORD"); //동일하게 비밀번호 취득

    //MySQL 서버에 접속하여 결과 표시

    try {
      $dsn = "mysql:host=$servername;dbname=$database";
      $conn = new PDO($dsn, $username, $password);
      $conn->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
      print("<p>접속에 성공했습니다.</p>");
    } catch(PDOException $e) {
      print("<p>접속에 실패했습니다.</p>");
      echo $e->getMessage();
    }

    $conn = null;
    print("<p>종료합니다.</p>");

    ?>
  </body>
</html>