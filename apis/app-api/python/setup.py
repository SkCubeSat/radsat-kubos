#!/usr/bin/env python3
"""
A setuptools based setup module.
See:
https://github.com/pypa/sampleproject
"""

from setuptools import setup, find_packages

setup(name='kubos_app',
      version='0.1.0',
      description='Mission Application API for KubOS',
      packages=find_packages(),
      install_requires=['toml']
      )
