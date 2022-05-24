# SkillCheck
Best way to destroy imposter syndrome is to build projects.


I intend to build the following projects in both Rust and Python, to prove to myself more than anything that I do actually know what I'm doing. 
This whole thing occurred because of how massive some people's cranium is on Stack Overflow, Discord, etc; and I've fallen into the 'intermediate perfectionist' rut of iterating over things dozens of times without a finished copy. So these projects MUST be done in 1-3 days each based on complexity. They should span the entirety of the language features, as well as a depth of technical knowledge to the absolute limit of my abilities or how I perceive them.

Beginner | Intermediate | Difficult | Challenging | Expert  ( also some Fun Projects )  
So, Onwards to the projects:  
Name| Difficulty  
Pi to the Nth digit : Beginner  
Fizz Buzz           : Beginner  
Prime factorization : Beginner  
SQL Query Validator : Intermediate  
Binary <> Decimal.. : Intermediate  
Bubble Sort         : Intermediate  
Alarm Clock/PDA     : Intermediate  
Number to Name Conv : Intermediate  
News "Scraper"      : Intermediate  
Static Code Counter : Intermediate  
Websocket Chat App  : Intermediate  
Budgeting Helper    : Intermediate  
Chess Engine        : Difficult  
Merge Sort          : Difficult  
Text Editor         : Difficult  
Bandwidth Monitor   : Difficult  
Packet Sniffer      : Difficult  
Sound Board         : Difficult  
SQL Query Analyzer  : Difficult  
Pixel Art Painter   : Difficult  
Code Snippet Manager: Challenging  
Inventory Management: Challenging  
Recipe Generator    : Challenging  
Flower Shop POS     : Challenging  
Movie Recommendation: Challenging  



Full Disclosure, Many of the ideas were taken from:
https://github.com/karan/Projects
So thank you to them for making the planning phase much easier. Descriptions below are influenced by karan's if they are available, but are written by me so check the links for better descriptions if mine don't make sense.

------------------------
Expanded Descriptions
------------------------

Pi to the Nth Digit - https://github.com/karan/Projects#numbers  
Create a program that can generate Pi reliably up to the first 1000 digits.

Prime Factorization - https://github.com/karan/Projects#numbers  
Have the user enter a whole number and find all the prime factors if any exist.

FizzBuzz - https://github.com/karan/Projects#text  
Print the numbers from 1 to 100 inclusively, but for all multiples of 3 print "Fizz" instead; likewise for multiples of 5 print "Buzz". For any intersection of these two cases, print "FizzBuzz".

SQL Query Validator - Frosttoys  
Have the user enter an SQL Query, and validate that it is proper syntactically. AKA a syntax validator.

Binary/Decimal Bi-directional Converter - https://github.com/karan/Projects#numbers  
Convert from common Binary representations (0b0000_0000 and 0b00000000) to Decimal, and from Decimal to Binary.

Bubble Sort - https://github.com/karan/Projects#classic-algorithms  
Iteratively sort over an array of sortable elements in place by comparing two elements and flipping them if necessary. O(n2)

Alarm Clock / PDA Assistant - https://github.com/karan/Projects#numbers / Frosttoys  
An alarm clock that alerts the user after a given or particular time. System time is fine for this. Upgrade this with a PDA that handles multiple events throughout the day over multiple days to help keep you on track.

Number Names (Number to English Name Converter) - https://github.com/karan/Projects#numbers  
Have the user enter a number, and provide the english written form of the number. Upgrade this by supporting negative, zero, or floating-point numbers.

News "Scraper" - Frosttoys  
Find a free news API and have a bot query for news through filters you can set. Great for Smart Mirrors!

Static Code Counter - Frosttoys  
Reads a source code file of a certain language type and counts how many lines of actual code there are.
Upgrade this with statistics like variable usage, function usage, and lines of comments.

Websocket Chat Application - Frosttoys  
An "IRC" like chat application based on the websocket framework.

Budgeting Helper - Frosttoys  
Takes user input about their income, spending, saving goals, etc, and produce a report that tells them where they are spending the most, how their savings will appreciate or depreciate at the current spending rate, and things like that. Upgrade with a GUI.

Chess Engine - Frosttoys  
Calculate possible moves without cheating, both in the game itself and in the algorithm that the chess "bot" uses. Upgrade this with a difficulty based algorithm like the ELO system.

Merge Sort - https://github.com/karan/Projects#classic-algorithms  
Merge sort is a recursive, "Divide and Conquer", type sorting algorithm. As the name suggests it breaks down the "container" of unsorted elements, and sorts them while merging back together. O(nlogn)

Text Editor - https://github.com/karan/Projects#text  
Notepad style text editor which can open, edit, and save text documents (.txt)
Upgrade this with Markdown language processing, Syntax highlighting, pixel-wise scrolling, an easy to use hotbar (optionally customizable) or anything else you can think of.

Bandwidth Monitor - https://github.com/karan/Projects#networking  
Watch the network interfaces attached for how much data was uploaded or downloaded during the course of the scanning scheduling. Upgrade this with graphing of the bandwidth usage over time

Packet Sniffer - Frosttoys  
Check what processes are sending and receiving and how much. As well as addressee and what their payload is. Upgrade this by integrating it with the bandwidth monitor and making flame graphs of each process over time.

Sound Board - Frosttoys  
Play sound files on keybind even if the program is not the primary window. Upgrade this with configurable keybinds (bonus points for multi-key binds), a nice GUI, a virtual sound device for added volume configurability, etc.

SQL Query Analyzer - Frosttoys / https://github.com/karan/Projects#databases  
A utility application that can run a query against a configurable database and revert, to validate the query works. Upgrade this with a query optimizer, that offers suggestions to make the query more efficient. Upgrade this by adding it to the Text editor and making Syntax Highlighting.

Code Snippet Manager - https://github.com/karan/Projects#files  
A utility to store snippets of code (Functions, Classes, interesting tidbits) into an organised format. Organise it however you like, some methods include Tagging, Partioning, or Database tables. Upgrade this with a GUI, Database, Syntax highlighting or the Text Editor from earlier!

Inventory Management - https://github.com/karan/Projects#classes (product inventory project)  
An application that manages an inventory of products, including incoming, outgoing, damaged, returned, and lost products. Upgrade this by adding a GUI or Database, a form filler would also be good for different reports that may be linked to specific products such as returned or damaged inventory.

Recipe Generator - https://github.com/karan/Projects#classes   
Creates a recipe from the given ingredients using a specific "Recipe" algorithm such as soup, salad, pizza. Upgrade this by storing favourite recipes, allowing tweaks to the recipe, having allergy/intolerance filters, etc.

Flower Shop POS - https://github.com/karan/Projects#classes  
Combines an inventory management solution with a transaction scheme to keep track of which flowers are available at what price and automatically calculate bouquets from the flowers used. Upgrade this by adding statistics, a database, a time logging feature, etc.

Movie Recommender - Frosttoys  
Recommend a movie based on mood, what they previously watched, and other factors to generate a list of movies they may enjoy.
