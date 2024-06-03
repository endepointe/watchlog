# log-collection-documentation
# Software Development Process README

## Table of Contents

1. [Introduction](#introduction)
2. [Development Phases](#development-phases)
3. [Roles and Responsibilities](#roles-and-responsibilities)
4. [Communication and Collaboration](#communication-and-collaboration)
5. [Tools and Technologies](#tools-and-technologies)
6. [Quality Assurance and Testing](#quality-assurance-and-testing)
7. [Project Timeline](#project-timeline)
8. [Risk Management](#risk-management)
9. [Documentation](#documentation)

## Introduction

This document provides a comprehensive guide to the development methodology, 
outlining activities, goals, roles, tools, testing procedures, timelines, 
risk management, documentation practices, and continuous improvement strategies.

The overall goal is to create a seemless, organized, workflow process that allows 
contributors to collaborate efficiently.

The software development process begins with ideation, where creative concepts
are generated on the backlog, followed by iterative refinement, testing, and
continuous enhancement, ultimately leading to the identification and
implementation of the most optimal solution among the iterations.

## Development Phases

### 1. Planning
The backlog board serves as a dynamic space where ideas are generated, refined,
and broken down into their sub-tasks. The over-arching ideas will start in the
TODO section whose sub-tasks (if any) will be moved to the In-Progress section
when defined. Team members contribute to discussions, prioritize features, and
outline the scope of the task on the backlog board. The backlog board will be the
central hub for refining concepts, setting milestones, and creating a transparent
and organized planning process as the foundation of the project's development.

### 2. Communication and Collaboration
The backlog board will serve as the central hub for project communication. Things
of a personal, non-project related discussions such as scheduling, course
meetings, etc, will be handled on Discord.

### 3. Development
The project will benefit from members iterating through several versions of code
before reaching a solution. Through this series of incremental improvements,
members can build upon the lessons learned previously, enhancing the project in a
meaningful way, while remaining adaptible to new insights discovered.

This iterative development approach (agile in a way), will lead to a codebase
that aims to be secure, resilient, and organized solution, ensuring that the
final product aligns with the user expectations.

### 4. Testing 
Through the discussions and iterations, members will write unit test that adhere
to industry standards.

### 5. Documentation
Code comments will be written to enhance the understanding when necessary.
If a discussion would better explain the code reason, provide a link as a comment
instead.

### 6. Adjustment and Feedback
The discussions about project direction and feature implementation will occur 
on the backlog board. In this way, maintainers to the project can have a full
view of the discussion history. Feedback regarding ideas is encouraged as this is
the only way to have a discussion and reach an optimal solution.

## Roles and Responsibilities

Due to the team size, all members are encourage to collabarate in every section
of the product. Repository admins are put in place to create stable development
and feature-testing branches.

As of now:
- Jicxer is the frontend admin.
- murchej is the server admin.
- endepointe is the client admin.
- khuynh2002 is the documentation admin.

## Tools and Technologies

The languages chosen for each repo may vary. For example, the client repository
language will be written in Rust while the frontend repo may chose a stack that
makes sense based on its own requirements.

Members should be comfortable moving between languages as requirements demand.

## Quality Assurance and Testing

The release cycle will follow **Major.Minor.Patch.**
- Major: Breaking changes
- Minor: New features
- Patch: Bug fixes

With the iterative approach taken to the project, quality assurance and testing
will be incorporated into the software development process. The ensure that the
software meets the required specifications, the following items should be
considered during discussion and implementation:

1. Analyze the requirements
2. Plan appropriate tests (security, performance, regression, user)
3. Execute those tests
4. Use the results of the test for further discussion
5. Release (Major.Minor.Patch) 
6. Repeat

## Project Timeline

A stable version of the product will take 9 months. Within 6 months, a minor
version should be available for further testing.

## Risk Management

Risks, hurdles, obstacles are to be expected. Through the discussions and
iterations, a solution-first approach can mitigate issues as they arise.

## Documentation

The user documentation will be included in this repository, and there are plans
to include it in either (readthedocs.io)[https://docs.readthedocs.io/] or another
platform.

The documentation for project development and maintenance will be included in the
README of each corresponding repository within the organization.

