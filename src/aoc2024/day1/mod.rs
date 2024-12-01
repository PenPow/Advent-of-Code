const INPUT: &str = include_str!("./input.txt");

pub fn solution() {
	let split = INPUT
		.trim()
		.split('\n')
		.collect::<Vec<&str>>();
	
	let (mut list1, mut list2): (Vec<u32>, Vec<u32>) = split
		.iter()
		.map(|str| str.split_whitespace().collect::<Vec<&str>>())
		.map(|pair| {
			let numbers = pair.iter().map(|number| number.parse::<u32>().unwrap()).collect::<Vec<u32>>();

			(numbers[0], numbers[1])
		})
		.unzip();

	// PART 1
	list1.sort();
	list2.sort();

	let total_distance = list1
		.iter()
		.zip(list2.clone())
		.map(|(num1, num2)| num1.abs_diff(num2))
		.collect::<Vec<u32>>()
		.iter()
		.sum::<u32>();

	println!("(Part 1) The total distance is {total_distance}");

	// PART 2
	let similarity = list1
		.iter()
		.map(|left| {
			let count = list2.iter().filter(|right| *left == **right).count() as u32;
			
			left * count
		})
		.sum::<u32>();

	println!("(Part 2) The similarity is {similarity}")
}