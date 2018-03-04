#include <iostream>
#include <vector>
#include <string>

int main()
{
	std::vector<std::string>* x = 0;

	{
		std::vector<std::string> z;

		z.push_back("Hello para//el 2018!");
		x = &z;
	}

	std::cout << (*x)[0] << std::endl;
}
