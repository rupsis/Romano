Feature: Color feature

  Scenario: Colors are [red, green, blue] tuples
    Given c <- color[-0.5, 0.4, 1.7]
    Then c.r = -0.5
    And c.g = 0.4
    And c.b = 1.7

# Operation

  Scenario: Adding colors
    Given c1 <- color[0.9, 0.6, 0.75]
    And c2 <- color[0.7, 0.1, 0.25]
    Then c1 + c2 = color[1.6, 0.7, 1.0]

  Scenario: Subtracting colors
    Given c1 <- color[0.9, 0.6, 0.75]
    And c2 <- color[0.7, 0.1, 0.25]
    Then c1 - c2 = color[0.2, 0.5, 0.5]

  Scenario: Multiplying a colors by a scalar
    Given c1 <- color[0.2, 0.3, 0.4]
    Then c1 * 2  = color[0.4, 0.6, 0.8]

  Scenario: Multiplying colors
    Given c1 <- color[1, 0.2, 0.4]
    And c2 <- color[0.9, 1, 0.1]
    Then c1 * c2 = color[0.9, 0.2, 0.04]

