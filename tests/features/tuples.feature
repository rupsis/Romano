Feature: Tuple feature

  Scenario: A tuple with w=1.0 is a Point
    Given a <- tuple[4.3, -4.2, 3.1, 1.0]
    Then a.x = 4.3
    And a.y = -4.2
    And a.z = 3.1
    And a.w = 1.0
    And a is a point
    And a is not a vector

  Scenario: A tuple with w=0 is a Vector
    Given a <- tuple[4.3, -4.2, 3.1, 0.0]
    Then a.x = 4.3
    And a.y = -4.2
    And a.z = 3.1
    And a.w = 0.0
    And a is a vector
    And a is not a point

  Scenario: point() creates tuples with w=1
    Given p <- point[4, -4, 3]
    Then p = tuple[4, -4, 3, 1]

  Scenario: vector() creates tuples with w=0
    Given p <- vector[4, -4, 3]
    Then p = tuple[4, -4, 3, 0]
